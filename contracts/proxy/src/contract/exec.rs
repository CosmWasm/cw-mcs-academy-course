use common::msg::membership::ExecMsg as MembershipExecMsg;
use cosmwasm_std::{
    coins, ensure, to_binary, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, Uint128,
    WasmMsg,
};
use cw_utils::must_pay;
use distribution::msg::ExecMsg as DistribtionExecMsg;

use crate::contract::{PROPOSE_MEMBER_ID, WITHDRAW_REPLY_ID};
use crate::error::ContractError;
use crate::state::{
    WithdrawalData, CONFIG, DONATIONS, HALFTIME, LAST_UPDATED, OWNER, PENDING_WITHDRAWAL, WEIGHT,
};

pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let amount = must_pay(&info, &config.denom)?;

    let direct_amount = config.direct_part * amount;
    let to_distribute = amount - direct_amount;

    let distribution_msg = DistribtionExecMsg::Distribute {};
    let distribution_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&distribution_msg)?,
        funds: coins(to_distribute.u128(), &config.denom),
    };

    let resp = Response::new()
        .add_message(distribution_msg)
        .add_attribute("action", "donate")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("amount", amount.to_string());

    DONATIONS.update(deps.storage, |donations| -> StdResult<_> {
        Ok(donations + 1)
    })?;

    Ok(resp)
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receiver: Option<String>,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    let weight = WEIGHT.load(deps.storage)?;
    let donations = DONATIONS.load(deps.storage)?;
    let diff = donations as i64 - weight as i64;
    WEIGHT.save(deps.storage, &donations)?;
    DONATIONS.save(deps.storage, &1)?;
    LAST_UPDATED.save(deps.storage, &env.block.time.seconds())?;

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());
    PENDING_WITHDRAWAL.save(deps.storage, &WithdrawalData { receiver, amount })?;

    let config = CONFIG.load(deps.storage)?;

    let withdraw_msg = DistribtionExecMsg::Withdraw { weight, diff };
    let withdraw_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&withdraw_msg)?,
        funds: vec![],
    };
    let withdraw_msg = SubMsg::reply_on_success(withdraw_msg, WITHDRAW_REPLY_ID);

    let resp = Response::new()
        .add_submessage(withdraw_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("new weight", weight.to_string());

    Ok(resp)
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.is_closed = true;
        Ok(config)
    })?;

    let resp = Response::new().add_attribute("action", "close");
    Ok(resp)
}

pub fn propose_member(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::Unauthorized);

    let config = CONFIG.load(deps.storage)?;

    let propose_msg = MembershipExecMsg::ProposeMember { addr: addr.clone() };
    let propose_msg = WasmMsg::Execute {
        contract_addr: config.membership_contract.into_string(),
        msg: to_binary(&propose_msg)?,
        funds: vec![],
    };
    let propose_msg = SubMsg::reply_on_success(propose_msg, PROPOSE_MEMBER_ID);

    let resp = Response::new()
        .add_submessage(propose_msg)
        .add_attribute("action", "propose member")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("member", addr);

    Ok(resp)
}

pub fn update_weight(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let last_updated = LAST_UPDATED.load(deps.storage)?;
    let halftime = HALFTIME.load(deps.storage)?;
    let resp = Response::new()
        .add_attribute("action", "update_weight")
        .add_attribute("sender", info.sender.as_str());

    let elapsed = env.block.time.seconds() - last_updated;
    if elapsed < halftime {
        let resp = resp.add_attribute("performed", "no");
        return Ok(resp);
    }

    let config = CONFIG.load(deps.storage)?;

    let resp = resp.add_attribute("performed", "yes");

    let weight = WEIGHT.load(deps.storage)?;
    let diff = -(weight as i64) / 2;

    let withdraw_msg = DistribtionExecMsg::Withdraw { weight, diff };
    let withdraw_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&withdraw_msg)?,
        funds: vec![],
    };

    let weight = (weight as i64 + diff) as u64;
    WEIGHT.save(deps.storage, &weight)?;

    let resp = resp
        .add_message(withdraw_msg)
        .add_attribute("new weight", weight.to_string());
    Ok(resp)
}

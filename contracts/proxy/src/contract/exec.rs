use cosmwasm_std::{
    coins, ensure, to_binary, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg,
};
use cw_utils::must_pay;

use crate::error::ContractError;
use crate::msg::DistribtionExecMsg;
use crate::state::{CONFIG, DONATIONS, OWNER};

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
    receiver: Option<String>,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    unimplemented!()
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

pub fn propose_member(deps: DepsMut, addr: String) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn update_weight(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    unimplemented!()
}

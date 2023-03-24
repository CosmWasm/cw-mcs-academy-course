use cosmwasm_std::{ensure, Decimal, DepsMut, Env, MessageInfo, Reply, Response};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, DONATIONS, HALFTIME, LAST_UPDATED, OWNER, WEIGHT};

mod exec;
mod reply;

const WITHDRAW_REPLY_ID: u64 = 1;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ensure!(
        Decimal::zero() <= msg.direct_part && msg.direct_part <= Decimal::percent(100),
        ContractError::InalidDirectPart
    );

    let owner = deps.api.addr_validate(&msg.owner)?;
    let distribution_contract = deps.api.addr_validate(&msg.distribution_contract)?;
    let membership_contract = deps.api.addr_validate(&msg.membership_contract)?;

    OWNER.save(deps.storage, &owner)?;
    WEIGHT.save(deps.storage, &msg.weight)?;
    DONATIONS.save(deps.storage, &1)?;
    CONFIG.save(
        deps.storage,
        &Config {
            denom: msg.denom,
            direct_part: msg.direct_part,
            distribution_contract,
            membership_contract,
            is_closed: false,
        },
    )?;
    HALFTIME.save(deps.storage, &msg.halftime)?;
    LAST_UPDATED.save(deps.storage, &env.block.time.seconds())?;

    let resp = Response::new();
    Ok(resp)
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;

    match msg {
        Donate {} => exec::donate(deps, info),
        Withdraw { receiver, amount } => exec::withdraw(deps, env, info, receiver, amount),
        Close {} => exec::close(deps, info),
        ProposeMember { addr } => exec::propose_member(deps, info, addr),
        UpdateWeight {} => exec::update_weight(deps, env, info),
    }
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        WITHDRAW_REPLY_ID => reply::withdraw(deps, env),
        id => Err(ContractError::UnrecognizedReplyId(id)),
    }
}

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg};
use crate::state::{MEMBERSHIP, TOTAL_WEIGHT};

pub mod exec;

pub const POINTS_SCALE: u128 = 4_000_000_000;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    TOTAL_WEIGHT.save(deps.storage, &msg.total_weigth)?;
    MEMBERSHIP.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;

    match msg {
        Distribute {} => exec::distribute(deps, env, info),
        NewMember { addr, weight } => exec::new_member(deps, env, info, addr, weight),
        Withdraw { weight, diff } => exec::withdraw(deps, env, info, weight, diff),
    }
}

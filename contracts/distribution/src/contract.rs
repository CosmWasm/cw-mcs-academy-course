use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg};
use crate::state::{MEMBERSHIP, TOTAL_WEIGHT};

mod exec;
mod query;

pub const POINTS_SCALE: u128 = 4_000_000_000;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    TOTAL_WEIGHT.save(deps.storage, &msg.total_weigth)?;
    MEMBERSHIP.save(deps.storage, &info.sender)?;
    let resp = Response::new().set_data(msg.data);
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
        Distribute {} => exec::distribute(deps, env, info),
        NewMember { addr, weight } => exec::new_member(deps, env, info, addr, weight),
        Withdraw { weight, diff } => exec::withdraw(deps, env, info, weight, diff),
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Withdrawable { proxy, weight } => {
            to_binary(&query::withdrawable(deps, env, proxy, weight)?)
        }
    }
}

use cosmwasm_std::{ensure, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use crate::error::ContractError;
use crate::state::{CONFIG, OWNER};

pub fn donate(deps: DepsMut) -> Result<Response, ContractError> {
    unimplemented!()
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

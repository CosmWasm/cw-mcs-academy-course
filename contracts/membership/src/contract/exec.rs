use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;

pub fn propose_member(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _addr: String,
) -> Result<Response, ContractError> {
    todo!()
}

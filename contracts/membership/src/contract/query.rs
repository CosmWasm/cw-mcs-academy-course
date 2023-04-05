use cosmwasm_std::{Addr, Deps, StdResult};

use crate::msg::IsMemberResp;

use common::state::membership::MEMBERS;

pub fn is_member(deps: Deps, addr: String) -> StdResult<IsMemberResp> {
    let is_member = MEMBERS.has(deps.storage, &Addr::unchecked(addr));

    Ok(IsMemberResp { is_member })
}

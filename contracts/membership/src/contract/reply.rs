use cosmwasm_std::{
    to_binary, Addr, DepsMut, Order, Response, StdError, StdResult, SubMsgResponse,
};
use cw_utils::parse_instantiate_response_data;

use crate::error::ContractError;
use crate::msg::{InstantationData, ProposeMemberData};
use crate::state::{AWAITING_INITIAL_RESPS, MEMBERS};

pub fn initial_proxy_instantiated(
    deps: DepsMut,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissing)?;
    let response = parse_instantiate_response_data(&data)?;
    let addr = Addr::unchecked(response.contract_address);

    MEMBERS.save(deps.storage, &addr, &cosmwasm_std::Empty {})?;

    let awaiting = AWAITING_INITIAL_RESPS.load(deps.storage)? - 1;
    if awaiting > 0 {
        AWAITING_INITIAL_RESPS.save(deps.storage, &0)?;

        let resp = Response::new().add_attribute("proxy_addr", addr);
        return Ok(resp);
    }

    let members: Vec<_> = MEMBERS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|member| -> StdResult<_> {
            let (member, _) = member?;
            let owner = proxy::state::OWNER.query(&deps.querier, member.clone())?;
            let data = ProposeMemberData {
                owner_addr: owner,
                proxy_addr: member,
            };
            Ok(data)
        })
        .collect::<StdResult<_>>()?;

    let data = InstantationData { members };
    let resp = Response::new()
        .add_attribute("proxy addr", addr.as_str())
        .set_data(to_binary(&data)?);

    Ok(resp)
}

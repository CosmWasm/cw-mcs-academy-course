use common::msg::ProposeMemberData;
use common::state::membership::MEMBERS;
use cosmwasm_std::{
    from_binary, to_binary, Addr, DepsMut, Env, Order, Response, StdError, StdResult, SubMsg,
    SubMsgResponse, WasmMsg,
};
use cw_utils::parse_instantiate_response_data;

use proxy::msg::InstantiateMsg as ProxyInstatiateMsg;

use crate::error::ContractError;
use crate::msg::InstantationData;
use crate::state::{AWAITING_INITIAL_RESPS, CONFIG};

use super::INITIAL_PROXY_INSTANTIATION_REPLY_ID;

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
        AWAITING_INITIAL_RESPS.save(deps.storage, &awaiting)?;

        let resp = Response::new().add_attribute("proxy_addr", addr);
        return Ok(resp);
    }

    let members: Vec<_> = MEMBERS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|member| -> StdResult<_> {
            let (member, _) = member?;
            let owner = proxy::state::OWNER.query(&deps.querier, member.clone())?;
            let data = ProposeMemberData {
                owner_addr: owner.into(),
                proxy_addr: member.into(),
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

pub fn proxy_instantiated(
    deps: DepsMut,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissing)?;
    let response = parse_instantiate_response_data(&data)?;
    let addr = Addr::unchecked(response.contract_address);

    let owner = proxy::state::OWNER.query(&deps.querier, addr.clone())?;

    MEMBERS.save(deps.storage, &addr, &cosmwasm_std::Empty {})?;

    let data = ProposeMemberData {
        owner_addr: owner.into(),
        proxy_addr: addr.to_string(),
    };

    let resp = Response::new()
        .add_attribute("proxy addr", addr.as_str())
        .set_data(to_binary(&data)?);

    Ok(resp)
}

pub fn distribution_instantiated(
    deps: DepsMut,
    env: Env,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissing)?;
    let response = parse_instantiate_response_data(&data)?;
    let initial_members: Vec<String> =
        from_binary(&response.data.ok_or(ContractError::DataMissing)?)?;

    let mut config = CONFIG.load(deps.storage)?;
    config.distribution_contract = Addr::unchecked(response.contract_address);
    CONFIG.save(deps.storage, &config)?;

    let msgs: Vec<_> = initial_members
        .into_iter()
        .map(|member| -> Result<_, ContractError> {
            let addr = deps.api.addr_validate(&member)?;
            let init_msg = ProxyInstatiateMsg {
                owner: addr.to_string(),
                weight: config.starting_weight,
                denom: config.denom.clone(),
                direct_part: config.direct_part,
                distribution_contract: config.distribution_contract.to_string(),
                membership_contract: env.contract.address.to_string(),
                halftime: config.halftime,
            };
            let msg = WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: config.proxy_code_id,
                msg: to_binary(&init_msg)?,
                funds: vec![],
                label: format!("{} Proxy", addr),
            };
            let msg = SubMsg::reply_on_success(msg, INITIAL_PROXY_INSTANTIATION_REPLY_ID);

            Ok(msg)
        })
        .collect::<Result<_, _>>()?;

    AWAITING_INITIAL_RESPS.save(deps.storage, &(msgs.len() as _))?;
    let resp = Response::new().add_submessages(msgs);

    Ok(resp)
}

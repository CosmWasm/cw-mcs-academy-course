use cosmwasm_std::{
    ensure, to_binary, Addr, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg};
use crate::state::{Config, AWAITING_INITIAL_RESPS, CONFIG};

use proxy::msg::InstantiateMsg as ProxyInstatiateMsg;

mod exec;
mod reply;

const INITIAL_PROXY_INSTANTIATION_REPLY_ID: u64 = 1;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ensure!(
        msg.minimal_acceptances >= 2,
        ContractError::NotEnoughRequiredAcceptances
    );

    ensure!(
        msg.minimal_acceptances <= msg.initial_members.len() as u64,
        ContractError::NotEnoughInitialMembers
    );

    let config = Config {
        starting_weight: msg.starting_weight,
        denom: msg.denom.clone(),
        direct_part: msg.direct_part,
        halftime: msg.halftime,
        proxy_code_id: msg.proxy_code_id,
        distribution_contract: Addr::unchecked(""),
        minimal_acceptances: msg.minimal_acceptances,
    };

    CONFIG.save(deps.storage, &config)?;

    let msgs: Vec<_> = msg
        .initial_members
        .into_iter()
        .map(|member| -> Result<_, ContractError> {
            let addr = deps.api.addr_validate(&member)?;
            let init_msg = ProxyInstatiateMsg {
                owner: addr.to_string(),
                weight: msg.starting_weight,
                denom: msg.denom.clone(),
                direct_part: msg.direct_part,
                distribution_contract: "".to_owned(),
                membership_contract: env.contract.address.to_string(),
                halftime: msg.halftime,
            };
            let msg = WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: msg.proxy_code_id,
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

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;

    match msg {
        ProposeMember { addr } => exec::propose_member(deps, env, info, addr),
    }
}

pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INITIAL_PROXY_INSTANTIATION_REPLY_ID => {
            reply::initial_proxy_instantiated(deps, reply.result.into_result())
        }
        id => Err(ContractError::UnrecognizedReplyId(id)),
    }
}

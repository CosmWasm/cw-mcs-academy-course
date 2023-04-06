use cosmwasm_std::{
    ensure, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsg, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

use distribution::msg::InstantiateMsg as DistributionInstantiateMsg;

mod exec;
mod query;
mod reply;

const INITIAL_PROXY_INSTANTIATION_REPLY_ID: u64 = 1;
const PROXY_INSTANTIATION_REPLY_ID: u64 = 2;
const DISTRIBUTION_INSTANTIATION_REPLY_ID: u64 = 3;

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

    let total_weigth = msg.starting_weight * msg.initial_members.len() as u64;
    let members_data = to_binary(&msg.initial_members)?;
    let instantiate_msg = DistributionInstantiateMsg {
        total_weigth: total_weigth.into(),
        data: members_data,
    };
    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: msg.distribution_code_id,
        msg: to_binary(&instantiate_msg)?,
        funds: vec![],
        label: "Distribution".to_owned(),
    };
    let instantiate_msg =
        SubMsg::reply_on_success(instantiate_msg, DISTRIBUTION_INSTANTIATION_REPLY_ID);

    let resp = Response::new().add_submessage(instantiate_msg);

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

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        IsMember { addr } => query::is_member(deps, addr).and_then(|resp| to_binary(&resp)),
    }
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INITIAL_PROXY_INSTANTIATION_REPLY_ID => {
            reply::initial_proxy_instantiated(deps, reply.result.into_result())
        }
        PROXY_INSTANTIATION_REPLY_ID => reply::proxy_instantiated(deps, reply.result.into_result()),
        DISTRIBUTION_INSTANTIATION_REPLY_ID => {
            reply::distribution_instantiated(deps, env, reply.result.into_result())
        }
        id => Err(ContractError::UnrecognizedReplyId(id)),
    }
}

use crate::contract::{execute, instantiate, query, reply};
use crate::msg::{ExecMsg, InstantationData, InstantiateMsg, IsMemberResp, QueryMsg};
use anyhow::Result as AnyResult;
use common::msg::ProposeMemberData;
use cosmwasm_std::{from_binary, to_binary, Addr, Decimal, WasmMsg};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::{parse_execute_response_data, parse_instantiate_response_data};

#[cfg(test)]
mod tests;

pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        CodeId(app.store_code(Box::new(contract)))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: &str,
        starting_weight: u64,
        denom: &str,
        direct_part: Decimal,
        halftime: u64,
        minimal_acceptances: u64,
        proxy_code_id: proxy::multitest::CodeId,
        distribution_code_id: distribution::multitest::CodeId,
        initial_members: &[&str],
        label: &str,
    ) -> AnyResult<(Contract, InstantationData)> {
        Contract::instantiate(
            app,
            self,
            sender,
            starting_weight,
            denom,
            direct_part,
            halftime,
            minimal_acceptances,
            proxy_code_id,
            distribution_code_id,
            initial_members,
            label,
        )
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl Contract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        starting_weight: u64,
        denom: &str,
        direct_part: Decimal,
        halftime: u64,
        minimal_acceptances: u64,
        proxy_code_id: proxy::multitest::CodeId,
        distribution_code_id: distribution::multitest::CodeId,
        initial_members: &[&str],
        label: &str,
    ) -> AnyResult<(Self, InstantationData)> {
        let msg = InstantiateMsg {
            starting_weight,
            denom: denom.to_owned(),
            direct_part,
            halftime,
            minimal_acceptances,
            proxy_code_id: proxy_code_id.into(),
            distribution_code_id: distribution_code_id.into(),
            initial_members: initial_members.iter().map(|s| s.to_string()).collect(),
        };

        let msg = WasmMsg::Instantiate {
            admin: None,
            code_id: code_id.0,
            msg: to_binary(&msg)?,
            funds: vec![],
            label: label.into(),
        };

        let res = app.execute(Addr::unchecked(sender), msg.into())?;
        let data = parse_instantiate_response_data(res.data.unwrap_or_default().as_slice())?;

        let contract = Self(Addr::unchecked(data.contract_address));
        let data = from_binary(&data.data.unwrap_or_default())?;

        Ok((contract, data))
    }

    #[track_caller]
    pub fn propose_member(
        &self,
        app: &mut App,
        sender: &str,
        addr: &str,
    ) -> AnyResult<Option<ProposeMemberData>> {
        let msg = ExecMsg::ProposeMember {
            addr: addr.to_owned(),
        };

        let resp = app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;

        resp.data
            .map(|data| parse_execute_response_data(&data))
            .transpose()?
            .and_then(|data| data.data)
            .map(|data| from_binary(&data))
            .transpose()
            .map_err(Into::into)
    }

    pub fn is_member(&self, app: &App, addr: &str) -> AnyResult<IsMemberResp> {
        let query = QueryMsg::IsMember {
            addr: addr.to_owned(),
        };

        app.wrap()
            .query_wasm_smart(self.0.clone(), &query)
            .map_err(Into::into)
    }
}

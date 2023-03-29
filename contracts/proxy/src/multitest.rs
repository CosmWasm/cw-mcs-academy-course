use anyhow::Result as AnyResult;
use common::msg::ProposeMemberData;
use cosmwasm_std::{from_binary, Addr, Coin, Decimal};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_utils::parse_execute_response_data;

use crate::contract::{execute, instantiate, query, reply};
use crate::msg::{ExecMsg, InstantiateMsg};

#[derive(Clone, Copy, Debug)]
pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        CodeId(app.store_code(Box::new(contract)))
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: &str,
        owner: &str,
        weight: u64,
        denom: &str,
        direct_part: Decimal,
        distribution_contract: &str,
        membership_contract: &str,
        halftime: u64,
        label: &str,
    ) -> AnyResult<Contract> {
        Contract::instantiate(
            app,
            self,
            sender,
            owner,
            weight,
            denom,
            direct_part,
            distribution_contract,
            membership_contract,
            halftime,
            label,
        )
    }
}

impl From<CodeId> for u64 {
    fn from(value: CodeId) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Contract(Addr);

impl Contract {
    pub fn from_addr(addr: Addr) -> Self {
        Self(addr)
    }

    pub fn addr(&self) -> &Addr {
        &self.0
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: CodeId,
        sender: &str,
        owner: &str,
        weight: u64,
        denom: &str,
        direct_part: Decimal,
        distribution_contract: &str,
        membership_contract: &str,
        halftime: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let msg = InstantiateMsg {
            owner: owner.to_owned(),
            weight,
            denom: denom.to_owned(),
            direct_part,
            distribution_contract: distribution_contract.to_owned(),
            membership_contract: membership_contract.to_owned(),
            halftime,
        };

        app.instantiate_contract(code_id.0, Addr::unchecked(sender), &msg, &[], label, None)
            .map(Self)
    }

    #[track_caller]
    pub fn donate(&self, app: &mut App, sender: &str, funds: &[Coin]) -> AnyResult<()> {
        let msg = ExecMsg::Donate {};

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, funds)?;
        Ok(())
    }

    #[track_caller]
    pub fn withdraw<'a>(
        &self,
        app: &mut App,
        sender: &str,
        receiver: impl Into<Option<&'a str>>,
        amount: impl Into<Option<u128>>,
    ) -> AnyResult<()> {
        let msg = ExecMsg::Withdraw {
            receiver: receiver.into().map(|s| s.to_owned()),
            amount: amount.into().map(|a| a.into()),
        };

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;
        Ok(())
    }

    #[track_caller]
    pub fn close(&self, app: &mut App, sender: &str) -> AnyResult<()> {
        let msg = ExecMsg::Close {};

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;
        Ok(())
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

    #[track_caller]
    pub fn update_weight(&self, app: &mut App, sender: &str) -> AnyResult<()> {
        let msg = ExecMsg::UpdateWeight {};

        app.execute_contract(Addr::unchecked(sender), self.0.clone(), &msg, &[])?;
        Ok(())
    }
}

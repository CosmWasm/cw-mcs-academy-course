use common::msg::WithdrawableResp;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub total_weigth: Uint128,
    pub data: Binary,
}

#[cw_serde]
pub enum ExecMsg {
    Distribute {},
    NewMember { addr: String, weight: u64 },
    Withdraw { weight: u64, diff: i64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(WithdrawableResp)]
    Withdrawable { proxy: String, weight: u64 },
}

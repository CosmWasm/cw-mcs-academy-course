use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub distribution_contract: String,
    pub membership_contract: String,
    pub halftime: u64,
}

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    Withdraw {
        receiver: Option<String>,
        amount: Option<Uint128>,
    },
    Close {},
    ProposeMember {
        addr: String,
    },
    UpdateWeight {},
}

#[cw_serde]
pub enum DistribtionExecMsg {
    Distribute {},
    Withdraw { weight: u64, diff: i64 },
}

#[cw_serde]
pub enum MembershipExecMsg {
    ProposeMember { addr: String },
}

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: Addr,
    pub proxy_addr: Addr,
}

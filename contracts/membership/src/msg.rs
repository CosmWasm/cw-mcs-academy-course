use common::msg::ProposeMemberData;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub minimal_acceptances: u64,
    pub proxy_code_id: u64,
    pub distribution_code_id: u64,
    pub initial_members: Vec<String>,
}

pub use common::msg::membership::*;

#[cw_serde]
pub struct InstantationData {
    pub members: Vec<ProposeMemberData>,
}

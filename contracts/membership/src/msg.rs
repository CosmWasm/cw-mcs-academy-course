use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub proxy_code_id: u64,
    pub distribution_code_id: u64,
}

#[cw_serde]
pub enum ExecMsg {
    ProposeMember { addr: String },
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

pub mod membership {
    use super::*;

    #[cw_serde]
    pub enum ExecMsg {
        ProposeMember { addr: String },
    }
}

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: String,
    pub proxy_addr: String,
}

#[cw_serde]
pub struct WithdrawableResp {
    pub funds: Vec<Coin>,
}

use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("{0}")]
    ParseError(#[from] ParseReplyError),
    #[error("Not enough initial members")]
    NotEnoughInitialMembers,
    #[error("Not enought required acceptances")]
    NotEnoughRequiredAcceptances,
    #[error("Unauthorised")]
    Unauthorized,
    #[error("Unrecognized reply id")]
    UnrecognizedReplyId(u64),
    #[error("Missing expected data")]
    DataMissing,
    #[error("Already voted on this proposal")]
    AlreadyVoted,
    #[error("Cannot propose a member")]
    AlreadyAMember,
}

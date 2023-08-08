use cosmwasm_std::StdError;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Custom error val: {val:?}")]
  CustomError{ val: String },

  #[error("Unauthorized")]
  Unauthorized {},

  #[error("no_state")]
  NoState {},

  #[error("only unordered channels are supported")]
  OrderedChannel {},

  #[error("invalid IBC channel version. Got ({actual}), expected ({expected})")]
  InvalidVersion { actual: String, expected: String },
}
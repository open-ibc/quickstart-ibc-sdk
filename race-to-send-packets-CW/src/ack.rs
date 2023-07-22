use cosmwasm_std::{to_binary, Binary};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Ack {
  Result(Binary),
  Error(String),
}

pub fn make_ack_success() -> Binary {
  let res = Ack::Result(b"1".into());
  to_binary(&res).unwrap()
}

pub fn make_ack_fail(err: String) -> Binary {
  let res = Ack::Error(err);
  to_binary(&res).unwrap()
}
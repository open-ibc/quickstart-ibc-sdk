use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};

#[cw_serde]
pub struct Config {
  pub admin_address: Addr,
}

#[cw_serde]
#[derive(Default)]
pub struct ChannelState {
  // count of messages successfully sent and received by counterparty
  pub count_sent: u32,
  // count of messages received
  pub count_received: u32,
  // latest received message
  pub latest_message: Option<String>,
}

#[cw_serde]
pub struct PacketData {
  pub poll_id: u8,
  pub voted: u8,
}

#[cw_serde]
pub struct Poll {
  pub one_option: u8,
  pub two_option: u8,
  pub three_option: u8,
  pub one_votes: u64,
  pub two_votes: u64,
  pub three_votes: u64,
  pub active_poll: bool,
  pub ibc_success: bool,
}

// map with channel_id as key and State as value
pub const CHANNEL_STATE: Map<String, ChannelState> = Map::new("channel_state");

pub const CONFIG: Item<Config> = Item::new("config");

pub const POLLS: Map<u8,Poll> = Map::new("polls");

pub const NEXT_ID : Item<u8> =Item::new("next_id"); 
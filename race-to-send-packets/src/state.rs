use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

#[cw_serde]
#[derive(Default)]
pub struct State {
  // count of messages successfully sent and received by counterparty
  pub count_sent: u32,
  // count of messages received
  pub count_received: u32,
  // latest received message
  pub latest_message: Option<String>,
}

// map with channel_id as key and State as value
pub const CHANNEL_STATE: Map<String, State> = Map::new("channel_state");

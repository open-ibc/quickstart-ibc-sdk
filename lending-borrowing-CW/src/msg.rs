use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Poll;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  SendMessage { channel: String, message: String },
  SendPollResult { channel: String, poll_id: u8, voted: u8 },
  CreatePoll { one_option: u8, two_option: u8, three_option: u8 },
  Vote { poll_id: u8, choice: u8},
  EndPoll { poll_id: u8 },
}

#[cw_serde]
pub enum IbcExecuteMsg {
  Message { message: String },
}

// Useful queries:
// 1. GetPoll, returns poll by id
// 2. GetActivePolls, returns all active polls you can vote on
// 3. GetPollsToSendIbc, returning all polls that are ended and still need IBC packet to be sent

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  /// Return the state for a particular channel
  #[returns(GetChannelStateResponse)]
  GetChannelState {
    // the ID of the channel to query its state
    channel: String,
  },
  // Return a poll by id
  #[returns(GetPollResponse)]
  GetPoll {
    poll_id: u8,
  },
}

#[cw_serde]
pub struct GetChannelStateResponse {
  pub count_sent: u32,
  pub count_received: u32,
  pub latest_message: Option<String>,
}

#[cw_serde]
pub struct GetPollResponse {
  pub poll: Option<Poll>,
}

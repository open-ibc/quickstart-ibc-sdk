use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  SendMessage { channel: String, message: String },
}

#[cw_serde]
pub enum IbcExecuteMsg {
  Message { message: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  /// Return the state for a particular channel
  #[returns(GetStateResponse)]
  GetState {
    // the ID of the channel to query its state
    channel: String,
  },
}

#[cw_serde]
pub struct GetStateResponse {
  pub count_sent: u32,
  pub count_received: u32,
  pub latest_message: Option<String>,
}
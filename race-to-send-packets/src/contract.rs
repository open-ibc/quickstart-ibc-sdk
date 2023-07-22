#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  to_binary, Binary, Deps, DepsMut, Env,
  IbcMsg, IbcTimeout, MessageInfo, Response, StdResult
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, IbcExecuteMsg, QueryMsg, GetStateResponse};
use crate::state::{State, CHANNEL_STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc-messenger";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

  Ok(Response::new()
    .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::SendMessage { channel, message} => execute::try_send_message(deps, env, channel, message),
  }
}

pub mod execute {
  use super::*;

  pub fn try_send_message(_deps: DepsMut, env: Env, channel: String, message: String) -> Result<Response, ContractError> {
    Ok(Response::new()
      .add_attribute("action", "send_message")
      .add_attribute("channel_id", channel.clone())
      // outbound IBC message, where packet is then received on other chain
      .add_message(IbcMsg::SendPacket {
        channel_id: channel,
        data: to_binary(&IbcExecuteMsg::Message { message })?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
      }))
  }

  /// Called on IBC packet receive in other chain
  pub fn try_receive_message(deps: DepsMut, channel: String, message: String) -> Result<State, ContractError> {
    CHANNEL_STATE.update(deps.storage, channel, |state| -> Result<_, ContractError> {
      match state {
        Some(mut s) => {
          s.count_received += 1;
          s.latest_message = Some(message);
          Ok(s)
        }
        None => Err(ContractError::NoState {}),
      }
    })
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetState { channel } => to_binary(&query_state(deps, channel.clone())?),
  }
}

pub fn query_state(deps: Deps, channel: String) -> StdResult<GetStateResponse> {
  let state = CHANNEL_STATE.load(deps.storage, channel.clone())?;
  Ok(GetStateResponse { 
    count_sent: state.count_sent, 
    count_received: state.count_received, 
    latest_message: state.latest_message 
  })
}

#[cfg(test)]
mod tests {}
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  to_binary, Binary, Deps, DepsMut, Env,
  IbcMsg, IbcTimeout, MessageInfo, Response, StdResult
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, IbcExecuteMsg, QueryMsg, GetChannelStateResponse, GetPollResponse};
use crate::state::{ChannelState, CHANNEL_STATE, CONFIG, Config, POLLS, Poll, NEXT_ID, PacketData};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc-poll-messenger";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

  let config = Config{
    admin_address: info.sender.clone(),
  };
  CONFIG.save(deps.storage, &config)?;

  NEXT_ID.save(deps.storage, &1)?;

  Ok(Response::new()
    .add_attribute("action", "instantiate")
    .add_attribute("admin_address", info.sender.clone())
    .add_attribute("next_id", "1")
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::SendMessage { channel, message} => execute::try_send_message(deps, env, channel, message),
    ExecuteMsg::SendPollResult { channel, poll_id, voted } => execute::try_send_poll_result(deps, env, channel, poll_id, voted),
    ExecuteMsg::CreatePoll { one_option, two_option, three_option } => execute::execute_create_poll(deps, env, info, one_option, two_option, three_option),
    ExecuteMsg::Vote { poll_id, choice } => execute::execute_vote(deps, env, info, poll_id, choice),
    ExecuteMsg::EndPoll { poll_id } => execute::execute_end_poll(deps, env, info, poll_id),
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
  pub fn try_receive_message(deps: DepsMut, channel: String, message: String) -> Result<ChannelState, ContractError> {
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

  pub fn try_send_poll_result(_deps: DepsMut, env: Env, channel: String, poll_id: u8 , voted: u8) -> Result<Response, ContractError> {
    let packet_data = PacketData {
      poll_id,
      voted,
    };

    Ok(Response::new()
      .add_attribute("action", "send_poll_result")
      .add_attribute("channel_id", channel.clone())
      // outbound IBC message, where packet is then received on other chain
      .add_message(IbcMsg::SendPacket {
        channel_id: channel,
        data: to_binary(&packet_data)?,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
      }))
  }

  pub fn execute_create_poll(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    one_option: u8, 
    two_option: u8, 
    three_option: u8
  ) -> Result<Response, ContractError> {
    // we load the config to check if the message sender is the admin
    // if not, throw error
    let config = CONFIG.load( deps.storage)?;

    if info.sender.clone() != config.admin_address {
      return Err(ContractError::Unauthorized {})
    }
    // we construct a poll and then store it in state
    let poll = Poll{
      one_option,
      two_option,
      three_option,
      one_votes: 0,
      two_votes: 0,
      three_votes: 0,
      active_poll: true,
      ibc_success: false,
    };

    POLLS.save(deps.storage, 1, &poll)?;

    // we must also update the next_id state param to have the correct id available when creating a new poll
    let mut next_id = NEXT_ID.load(deps.storage)?;
    next_id = next_id + 1;
    NEXT_ID.save(deps.storage, &next_id)?;

    Ok(Response::new()
      .add_attribute("action", "create_poll")
      .add_attribute("poll_id", (next_id-1).to_string())
    )
  }

  pub fn execute_vote(deps: DepsMut, _env: Env, _info: MessageInfo, poll_id: u8, choice: u8) -> Result<Response, ContractError> {
    // first we need to check if the inputs are valid
    // 1. is there a poll with this id?
    // 2. is the poll active and eligible to be voted on?
    // 3. is the choice one of the correct choices?
    // if the input is validated, we can tally the vote
    if !POLLS.has(deps.storage, poll_id) {
      return Err(ContractError::CustomError { val: "There is no poll with this ID".to_string() })
    }
    
    let mut poll = POLLS.load(deps.storage, poll_id)?;

    if !poll.active_poll {
      return Err(ContractError::CustomError { val: ("The poll has ended").to_string() })
    }

    if choice == poll.one_option {
      poll.one_votes += 1;
    } else if choice == poll.two_option {
      poll.two_votes += 1;
    } else if choice == poll.three_option {
      poll.three_votes += 1;
    } else {
      return Err(ContractError::CustomError { val: ("No such voting option").to_string() })
    }

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new()
      .add_attribute("action", "vote")
      .add_attribute("poll_id", poll_id.clone().to_string())
      .add_attribute("choice", choice.clone().to_string())
    )
  }

  pub fn execute_end_poll(deps: DepsMut, _env: Env, info: MessageInfo, poll_id: u8) -> Result<Response, ContractError> {
    // Requirements:
    // 1. only the admin is allowed to end the vote
    // 2. the poll must exist
    // 3. at least one person must have voted

    // check if sender of message is admin
    let config = CONFIG.load( deps.storage)?;

    if info.sender.clone() != config.admin_address {
      return Err(ContractError::Unauthorized {})
    }
    // check if the poll id exists
    if !POLLS.has(deps.storage, poll_id) {
      return Err(ContractError::CustomError { val: "There is no poll with this ID".to_string() })
    }
    
    let mut poll = POLLS.load(deps.storage, poll_id)?;
    // check if at least one person voted:
    if poll.one_votes + poll.two_votes + poll.three_votes == 0 {
      return Err(ContractError::CustomError { val: "Cannot end a poll without any votes being cast".to_string() })
    }

    // when all checks pass, then we can set the poll to inactive
    poll.active_poll = false;

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new()
      .add_attribute("action", "end_poll")
      .add_attribute("poll_id", poll_id.to_string())
    )
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetChannelState { channel } => to_binary(&query_state(deps, channel.clone())?),
    QueryMsg::GetPoll { poll_id } => to_binary(&query_poll(deps, poll_id.clone())?),
  }
}

pub fn query_state(deps: Deps, channel: String) -> StdResult<GetChannelStateResponse> {
  let state = CHANNEL_STATE.load(deps.storage, channel.clone())?;
  Ok(GetChannelStateResponse { 
    count_sent: state.count_sent, 
    count_received: state.count_received, 
    latest_message: state.latest_message 
  })
}

pub fn query_poll(deps: Deps, poll_id: u8) -> StdResult<GetPollResponse> {
  let queried_poll = POLLS.may_load(deps.storage, poll_id)?;
  Ok(GetPollResponse { poll: queried_poll.clone() })
}

#[cfg(test)]
mod tests {}
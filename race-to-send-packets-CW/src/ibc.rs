#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  from_binary, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
  IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
  IbcPacketTimeoutMsg, IbcReceiveResponse,
};

use crate::{
  ack::{make_ack_fail, make_ack_success, Ack},
  contract::execute::try_receive_message,
  error::Never,
  msg::IbcExecuteMsg,
  ContractError,
  state::{State, CHANNEL_STATE},
};

pub const IBC_VERSION: &str = "messenger-1";

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
  _deps: DepsMut,
  _env: Env,
  msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
  validate_order_and_version(msg.channel(), msg.counterparty_version())
}

/// Handles the `OpenAck` and `OpenConfirm` parts of the IBC handshake
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
  deps: DepsMut,
  _env: Env,
  msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
  validate_order_and_version(msg.channel(), msg.counterparty_version())?;

  // initialize the state for this channel
  let channel = msg.channel().endpoint.channel_id.clone();
  let state = State {
    count_sent: 0,
    count_received: 0,
    latest_message: None,
  };
  CHANNEL_STATE.save(deps.storage, channel.clone(), &state)?;

  Ok(IbcBasicResponse::new()
    .add_attribute("action", "channel_connect")
    .add_attribute("channel_id", channel.clone()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
  _deps: DepsMut,
  _env: Env,
  _msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
  unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
  deps: DepsMut,
  env: Env,
  msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
  match do_ibc_packet_receive(deps, env, msg) {
    Ok(response) => Ok(response),
    Err(error) => Ok(IbcReceiveResponse::new()
      .add_attribute("action", "packet_receive")
      .add_attribute("error", error.to_string())
      // on error write an error ack
      .set_ack(make_ack_fail(error.to_string()))),
  }
}

pub fn do_ibc_packet_receive(
  deps: DepsMut,
  _env: Env,
  msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
  // the channel ID this packet is being relayed along on this chain
  let channel = msg.packet.dest.channel_id.clone();
  let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

  match msg {
    IbcExecuteMsg::Message { message } => execute_receive_message(deps, channel, message),
  }
}

fn execute_receive_message(deps: DepsMut, channel: String, message: String) -> Result<IbcReceiveResponse, ContractError> {
  let state = try_receive_message(deps, channel.clone(), message)?;
  Ok(IbcReceiveResponse::new()
    .add_attribute("action", "receive_message")
    .add_attribute("channel_id", channel.clone())
    .add_attribute("count_received", state.count_received.to_string())
    .add_attribute("latest_message", state.latest_message.unwrap_or_default())
    // write a success ack
    .set_ack(make_ack_success()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
  deps: DepsMut,
  _env: Env,
  ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
  // load channel state
  let channel = ack.original_packet.src.channel_id.clone();
  let mut state = CHANNEL_STATE.load(deps.storage, channel.clone())?;

  // check acknowledgement data
  let acknowledgement: Ack = from_binary(&ack.acknowledgement.data)?;
  match acknowledgement {
    // for a success ack we increment the count of sent messages and save state
    Ack::Result(_) => {            
        state.count_sent += 1;
        CHANNEL_STATE.save(deps.storage, channel.clone(), &state)?;
    },
    // for an error ack we don't do anything and let the count of sent messages as it was
    Ack::Error(_) => {},
  }

  Ok(IbcBasicResponse::new()
    .add_attribute("action", "acknowledge")
    .add_attribute("channel_id", channel.clone())
    .add_attribute("count_sent", state.count_sent.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
  _deps: DepsMut,
  _env: Env,
  _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
  Ok(IbcBasicResponse::new().add_attribute("action", "timeout"))
}

pub fn validate_order_and_version(
  channel: &IbcChannel,
  counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
  // We expect an unordered channel here. Ordered channels have the
  // property that if a message is lost the entire channel will stop
  // working until you start it again.
  if channel.order != IbcOrder::Unordered {
    return Err(ContractError::OrderedChannel {});
  }

  if channel.version != IBC_VERSION {
    return Err(ContractError::InvalidVersion {
      actual: channel.version.to_string(),
      expected: IBC_VERSION.to_string(),
    });
  }

  // Make sure that we're talking with a counterparty who speaks the
  // same "protocol" as us.
  //
  // For a connection between chain A and chain B being established
  // by chain A, chain B knows counterparty information during
  // `OpenTry` and chain A knows counterparty information during
  // `OpenAck`. We verify it when we have it but when we don't it's
  // alright.
  if let Some(counterparty_version) = counterparty_version {
    if counterparty_version != IBC_VERSION {
      return Err(ContractError::InvalidVersion {
        actual: counterparty_version.to_string(),
        expected: IBC_VERSION.to_string(),
      });
    }
  }

  Ok(())
}
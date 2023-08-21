#!/bin/zsh

# Check if you got the message from ETH that collateral has been supplied:

echo 'Querying the channel state to see if the last message was confirmation of collateral being supplied \n'
export QUERY_GET_CHANNEL_STATE='{"get_channel_state":{"channel":"channel-0"}}'
ibctl exec wasm wasmd q wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $QUERY_GET_CHANNEL_STATE


# Admin creates a poll
echo 'The admin now creates a poll \n'
export EXECUTE_CREATE_POLL='{"create_poll":{"one_option":1,"two_option":2,"three_option":3}}'
ibctl exec wasm wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $EXECUTE_CREATE_POLL -- --from wasm158z04naus5r3vcanureh7u0ngs5q4l0gkwegr4 --keyring-backend test --chain-id wasm --gas auto --gas-adjustment 1.2 --yes

# Query the poll to see if successful
echo 'We query the poll... \n'
export QUERY_GET_POLL='{"get_poll":{"poll_id":1}}'
echo $(ibctl exec wasm wasmd q wasm contract-state smart wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $QUERY_GET_POLL)

sleep 10

# Any account can pass a vote
echo 'A random voter votes for the 3rd option... \n'
export EXECUTE_VOTE='{"vote":{"poll_id":1, "choice":3}}'
ibctl exec wasm wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $EXECUTE_VOTE -- --from wasm158z04naus5r3vcanureh7u0ngs5q4l0gkwegr4 --keyring-backend test --chain-id wasm --gas auto --gas-adjustment 1.2 --yes

sleep 10

# Only the admin can end the poll
echo 'The admin closes the poll... \n'
export EXECUTE_END_POLL='{"end_poll":{"poll_id":1}}'
ibctl exec wasm wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $EXECUTE_END_POLL -- --from wasm158z04naus5r3vcanureh7u0ngs5q4l0gkwegr4 --keyring-backend test --chain-id wasm --gas auto --gas-adjustment 1.2 --yes

sleep 10

# When the poll has ended AND a channel is created, we can send the IBC packet with poll info
echo 'The poll result is being sent over the IBC channel to trigger a loan on ETH... \n'
export EXECUTE_SEND_POLL_RESULT='{"send_poll_result":{"channel":"channel-0", "poll_id": 1, "voted": 3}}'
ibctl exec wasm wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d $EXECUTE_SEND_POLL_RESULT -- --from wasm158z04naus5r3vcanureh7u0ngs5q4l0gkwegr4 --keyring-backend test --chain-id wasm --gas auto --gas-adjustment 1.2 --yes

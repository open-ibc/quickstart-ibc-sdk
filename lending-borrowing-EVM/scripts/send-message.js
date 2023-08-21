// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const bytes32 = require('bytes32');

const dispatcherAddress = '0x0359d4C7309d1F171f01630807A68C367a05e1fb';

async function main() {
  //...
  /**
   * Here we start the interaction with the contracts!!!
   *
   * We'll execute the following:
   * 1. A transfer of PLYM tokens to check if contract interaction works
   * 2. We'll supply some funds of all token types to the lending/borrowing contract
   * 3. We'll put up some collateral with the account we'll borrow with
   * 4. We'll borrow one of the tokens.
   * 5. Lastly, we'll try sending an IBC packet
   */
  const accounts = await hre.ethers.getSigners();

  const ibcLendingBorrowing = await hre.ethers.getContractAt(
    'IbcLendingBorrowing',
    '0x37FA111284FBF97318CBe05C37fEC69071Dd4965'
  );
  // Now we'll send an IBC Packet with a message.
  // !!! Make sure a channel has been created (channel-0 is hardcoded)

  const fee = {
    recvFee: 0,
    ackFee: 0,
    timeoutFee: 0,
  };

  const tx = await ibcLendingBorrowing.sendMessage(
    dispatcherAddress,
    '{"message": {"message": "Collateral has been supplied"}}',
    bytes32({ input: 'channel-0' }),
    ((Date.now() + 60 * 60 * 1000) * 1000000).toString(),
    fee
  );

  await new Promise((r) => setTimeout(r, 60000));

  console.log(tx);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

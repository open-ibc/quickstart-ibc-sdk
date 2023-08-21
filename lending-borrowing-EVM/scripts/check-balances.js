// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const bytes32 = require('bytes32');

async function main() {
  const accounts = await hre.ethers.getSigners();

  // const polymer = await hre.ethers.getContractAt(
  //   'PolymerToken',
  //   '0x37FA111284FBF97318CBe05C37fEC69071Dd4965'
  // );

  // const openIbc = await hre.ethers.getContractAt(
  //   'OpenIbcToken',
  //   '0x85E9BbfE1BC5f87e6dE8b60fbcbE8DDE9EA12c0C'
  // );

  const ibcGlobal = await hre.ethers.getContractAt(
    'IbcGlobalToken',
    '0xf5c95209818EFAB8162008f4dE236A972eBA68a1'
  );

  const ibcLendingBorrowing = await hre.ethers.getContractAt(
    'IbcLendingBorrowing',
    '0xB10c73e50B9bdB51f3504F7104a411174B9C3aa3'
  );

  console.log(
    'After receiving the IBC packet and borrowing, $IBCG balance: ',
    await ibcGlobal.balanceOf(accounts[3])
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

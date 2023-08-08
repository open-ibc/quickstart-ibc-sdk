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

  // Polymer Token deployment
  const polymer = await hre.ethers.deployContract('PolymerToken', accounts[1]);
  await polymer.waitForDeployment();

  console.log('Polymer Token address:', polymer.target);

  // OpenIBC Token deployment
  const openIbc = await hre.ethers.deployContract('OpenIbcToken', accounts[2]);
  await openIbc.waitForDeployment();

  console.log('OpenIBC Token address:', openIbc.target);

  // Nebular Token deployment
  const nebular = await hre.ethers.deployContract('NebularToken', accounts[3]);
  await nebular.waitForDeployment();

  console.log('Nebular Token address:', nebular.target);

  // IBC enabled lending/borrowing contract deployment
  const ibcLendingBorrowing = await hre.ethers.deployContract(
    'IbcLendingBorrowing',
    [polymer.target, openIbc.target, nebular.target]
  );
  await ibcLendingBorrowing.waitForDeployment();

  console.log('Lending/Borrowing address:', ibcLendingBorrowing.target);

  // Now all contracts have been deployed,
  // make sure to copy and update the contract addresses in scripts/interact.js

  console.log('Address to receive the loan: ', accounts[4].address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

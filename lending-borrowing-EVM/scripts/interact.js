// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require('hardhat');
const bytes32 = require('bytes32');

async function main() {
  //...
  /**
   * Here we start the interaction with the contracts!!!
   *
   * We'll execute the following:
   * 1. A transfer of PLYM tokens to check if contract interaction works
   * 2. We'll supply some funds of all token types to the lending/borrowing contract
   * 3. We'll put up some collateral with the account we'll borrow with
   * 4. We'll borrow one of the tokens. (when boolean is turned on)
   * 5. Lastly, we'll try sending an IBC packet (send-message.js script)
   */
  const accounts = await hre.ethers.getSigners();

  const polymer = await hre.ethers.getContractAt(
    'PolymerToken',
    '0x85E9BbfE1BC5f87e6dE8b60fbcbE8DDE9EA12c0C'
  );

  const openIbc = await hre.ethers.getContractAt(
    'OpenIbcToken',
    '0xdcAD6B6CcabAdd7B3078F07fe80AcAeA279dBcbC'
  );

  const ibcGlobal = await hre.ethers.getContractAt(
    'IbcGlobalToken',
    '0xf5c95209818EFAB8162008f4dE236A972eBA68a1'
  );

  const ibcLendingBorrowing = await hre.ethers.getContractAt(
    'IbcLendingBorrowing',
    '0x37FA111284FBF97318CBe05C37fEC69071Dd4965'
  );

  console.log(
    'Before transferring, balance in PLYM for account0: ',
    await polymer.balanceOf(accounts[1].address)
  );

  // Just a test to see if contract interaction works
  const resp = await polymer
    .connect(accounts[1])
    .transfer(accounts[3].address, 345);
  await new Promise((r) => setTimeout(r, 20000));

  // Supply funds to the lending/borrowing contract for all 3 token types, use half of total supply

  const contracts = [ibcLendingBorrowing, polymer, openIbc, ibcGlobal];
  const totalSupply = [
    await ibcLendingBorrowing.totalSupply(),
    await polymer.totalSupply(),
    await openIbc.totalSupply(),
    await ibcGlobal.totalSupply(),
  ];

  console.log(
    'Before supplying funds, balance in PLYM for account0: ',
    await polymer.balanceOf(accounts[1].address)
  );

  // fund PLYM tokens
  await ibcLendingBorrowing
    .connect(accounts[1])
    .supplyFunds(contracts[1].getAddress(), totalSupply[1] / BigInt(2));

  // fund OIBC tokens
  await ibcLendingBorrowing
    .connect(accounts[2])
    .supplyFunds(contracts[2].getAddress(), totalSupply[2] / BigInt(2));

  // fund OIBC tokens
  await ibcLendingBorrowing
    .connect(accounts[2])
    .supplyFunds(contracts[3].getAddress(), totalSupply[3] / BigInt(2));

  await new Promise((r) => setTimeout(r, 20000));

  console.log(
    'After supplying funds, balance in PLYM for account1: ',
    await polymer.balanceOf(accounts[0].address)
  );

  // Supply collateral with the account you want to borrow with
  console.log(
    'Before supplying collateral, balance in LBTK for account4: ',
    await ibcLendingBorrowing.balanceOf(accounts[3].address)
  );

  await ibcLendingBorrowing
    .connect(accounts[3])
    .supplyCollateral({ value: hre.ethers.parseEther('25.0') });

  await new Promise((r) => setTimeout(r, 20000));

  console.log(
    'After supplying collateral, balance in LBTK for account4: ',
    await ibcLendingBorrowing.balanceOf(accounts[3].address)
  );

  // Now we can take out a loan, note that this is not executed as we will take out the loan later by sending an IBC packet
  // the function remains here only to test functionality locally before moving to cross-chain testing
  const executeLoan = false;
  if (executeLoan) {
    await ibcLendingBorrowing
      .connect(accounts[3])
      .borrow(
        contracts[1].getAddress(),
        await ibcLendingBorrowing.balanceOf(accounts[3].address)
      );

    await new Promise((r) => setTimeout(r, 20000));

    console.log(
      'After borrowing, balance in OIBC for account3: ',
      await contracts[1].balanceOf(accounts[3].address)
    );
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

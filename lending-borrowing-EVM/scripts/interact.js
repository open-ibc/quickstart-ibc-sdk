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
   * 5. Lastly, we'll try sending an IBC packet (check-balances.js script)
   */
  const accounts = await hre.ethers.getSigners();

  const polymer = await hre.ethers.getContractAt(
    'PolymerToken',
    '0x37FA111284FBF97318CBe05C37fEC69071Dd4965'
  );

  const openIbc = await hre.ethers.getContractAt(
    'OpenIbcToken',
    '0x85E9BbfE1BC5f87e6dE8b60fbcbE8DDE9EA12c0C'
  );

  const nebular = await hre.ethers.getContractAt(
    'NebularToken',
    '0xdcAD6B6CcabAdd7B3078F07fe80AcAeA279dBcbC'
  );

  const ibcLendingBorrowing = await hre.ethers.getContractAt(
    'IbcLendingBorrowing',
    '0xB10c73e50B9bdB51f3504F7104a411174B9C3aa3'
  );

  console.log(
    'Before transferring, balance in PLYM for account1: ',
    await polymer.balanceOf(accounts[1].address)
  );

  // Just a test to see if contract interaction works
  const resp = await polymer
    .connect(accounts[1])
    .transfer(accounts[0].address, 345);
  await new Promise((r) => setTimeout(r, 20000));

  // Supply funds to the lending/borrowing contract for all 3 token types, use half of total supply

  const contracts = [ibcLendingBorrowing, polymer, openIbc, nebular];
  const totalSupply = [
    await ibcLendingBorrowing.totalSupply(),
    await polymer.totalSupply(),
    await openIbc.totalSupply(),
    await nebular.totalSupply(),
  ];

  console.log(
    'Before supplying funds, balance in PLYM for account1: ',
    await polymer.balanceOf(accounts[1].address)
  );

  for (let i = 1; i < 4; i++) {
    let account = accounts[i];
    const response = await ibcLendingBorrowing
      .connect(account)
      .supplyFunds(contracts[i].getAddress(), totalSupply[i] / BigInt(2));
  }

  await new Promise((r) => setTimeout(r, 20000));

  console.log(
    'After supplying funds, balance in PLYM for account1: ',
    await polymer.balanceOf(accounts[1].address)
  );

  // Supply collateral with the account you want to borrow with
  console.log(
    'Before supplying collateral, balance in LBTK for account4: ',
    await ibcLendingBorrowing.balanceOf(accounts[4].address)
  );

  await ibcLendingBorrowing
    .connect(accounts[4])
    .supplyCollateral({ value: hre.ethers.parseEther('25.0') });

  await new Promise((r) => setTimeout(r, 20000));

  console.log(
    'After supplying collateral, balance in LBTK for account4: ',
    await ibcLendingBorrowing.balanceOf(accounts[4].address)
  );

  // Now we can take out a loan
  const executeLoan = false;
  if (executeLoan) {
    await ibcLendingBorrowing
      .connect(accounts[4])
      .borrow(
        contracts[2].getAddress(),
        await ibcLendingBorrowing.balanceOf(accounts[4].address)
      );

    await new Promise((r) => setTimeout(r, 20000));

    console.log(
      'After borrowing, balance in OIBC for account4: ',
      await contracts[2].balanceOf(accounts[4].address)
    );
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

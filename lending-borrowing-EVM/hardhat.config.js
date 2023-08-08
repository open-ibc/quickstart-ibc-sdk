require('@nomicfoundation/hardhat-toolbox');

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: '0.8.19',
  networks: {
    localibcsdk: {
      url: 'http://127.0.0.1:32793',
      accounts: [
        '0x826cccccf88094269e637c816d8895f138b89e03dfa2fdd8b5d9e1feea1cb9aa',
        '0x15188f87d4fd462b13c8f3b81c3a818ceb68fb596da273d6b7ee9f05f588e207',
        '0x75558cf96f6f28bb489fd33cbfc38aa2311bcb6586a9742f9586da809dd57fe2',
        '0xea6ad02a06e84b195f65a7e01ab32440a8914e523d53be71aba370167ce94ae9',
        '0xbaeb0652f541c24abdf69216fec5136bda1a013dea71ab24bb3b477143efa9ef',
      ],
    },
  },
};

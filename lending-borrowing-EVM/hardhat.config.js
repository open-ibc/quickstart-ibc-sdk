require('@nomicfoundation/hardhat-toolbox');
require('dotenv').config();

const endpoint = process.env.ENDPOINT || 'http://127.0.0.1:32777';
console.log(`Hardhat network listening at: ${endpoint}`);

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: '0.8.19',
  networks: {
    localibcsdk: {
      url: endpoint,
      // These private keys reflect the accounts that are generated (and funded)
      // by the default config file (minus the first account that is used as the relayer account internally)
      accounts: [
        '0x15188f87d4fd462b13c8f3b81c3a818ceb68fb596da273d6b7ee9f05f588e207',
        '0x75558cf96f6f28bb489fd33cbfc38aa2311bcb6586a9742f9586da809dd57fe2',
        '0xea6ad02a06e84b195f65a7e01ab32440a8914e523d53be71aba370167ce94ae9',
        '0xbaeb0652f541c24abdf69216fec5136bda1a013dea71ab24bb3b477143efa9ef',
      ],
    },
  },
};

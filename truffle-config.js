module.exports = {
  networks: {
    "develop": {
      accounts: 5
    },
    loc_test1_test1: {
      network_id: "*",
      port: 8545,
      host: "127.0.0.1"
    }
  },
  mocha: {},
  compilers: {
    solc: {
      version: "0.8.0",
      settings: {
        optimizer: {
          enabled: true,
          runs: 500000
        },
        evmVersion: "istanbul"
      }
    }
  },
  db: {
    enabled: false
  }
};

// const SafeMath = artifacts.require("SafeMath");
const Type1 = artifacts.require("Type1");
const BIC = artifacts.require("BIC");
const Swiftrange = artifacts.require("Swiftrange");
const Type2 = artifacts.require("Type2");
const Type2P = artifacts.require("Type2P");
const Type3 = artifacts.require("Type3");
const Type4 = artifacts.require("Type4");
const Flashproof = artifacts.require("Flashproof");
const BPPP = artifacts.require("BPPP");

module.exports = function (deployer) {
    deployer.then(async()=>{
        // await deployer.deploy(SafeMath);
        // await deployer.link(SafeMath, [Flashproof]);

        // await deployer.deploy(Type1);
        // await deployer.deploy(BIC);
        // await deployer.deploy(Swiftrange);
        // await deployer.deploy(Type2);
        await deployer.deploy(Type2P);
        // await deployer.deploy(Type3);
        // await deployer.deploy(Type4);
        // await deployer.deploy(Flashproof);
        // await deployer.deploy(BPPP);
    });
};

const { connect, keyStores, utils, Contract  } = require('near-api-js');
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
const myKeyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
const connectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore, 
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
};
const path = require('path');
const fs = require('fs');
const dotenv = require('dotenv');

const getContractName = () => {
    const envPath = path.resolve('../../neardev/dev-account.env');
    const envConfig = dotenv.parse(fs.readFileSync(envPath));

    dotenv.config({ path: envPath });

    return process.env.CONTRACT_NAME;
}

const nearConnection = async () => {
    await connect(connectionConfig);
};

const tranferYoctoNearToNear = (amount) => {
    return utils.format.formatNearAmount(amount);
}

const getAccount = async (accountId) => {
    return await (await connect(connectionConfig)).account(accountId);
}

const createContract = (accountId, contractId) => {
    const methodOptions = {
        viewMethods: ['get_all_works'],
        changeMethods: ['create_work']
    };

    const contract = new Contract(
        accountId,
        contractId,
        methodOptions
    );

    return contract;
}

const create_work = async (accountId, name, content, fee=0) => {
    let contractId = getContractName();

    let contract = createContract(accountId, contractId);
    
    return await contract.create_work(
        {
            callbackUrl: content,
            args: {
                name, content, fee
            },
        }
    );
}

module.exports = { nearConnection, tranferYoctoNearToNear, getAccount, create_work };
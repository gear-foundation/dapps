const { GearApi, GearKeyring, getWasmMetadata } = require('@gear-js/api');
const { readFileSync } = require('fs');

require('dotenv').config();

const uploadProgram = async (
    api,
    pathToProgram,
    pathToMeta,
    account,
    value,
    initPayload) => {
    const code = readFileSync(pathToProgram);
    const metaFile = pathToMeta ? readFileSync(pathToMeta) : undefined;
    const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;
    const gas = await api.program.gasSpent.init(
        account.publicKey,
        code,
        initPayload,
        value,
        meta
      );
    console.log("GAS SPENT", gas.toHuman());
    const programId = api.program.submit({ code, initPayload, gasLimit: gas }, meta);
    await api.program.signAndSend(account, (data) => {
        console.log(data.toHuman());
    });
    return programId;
}

async function main() {
    const gearApi = await GearApi.create();
    const jsonKeyring = readFileSync('./account.json').toString();;
    const account = GearKeyring.fromJson(jsonKeyring, 'Google06!!');
    console.log("start deploying program");

    console.log(process.env.OPT_WASM)
    let program = await uploadProgram(
        gearApi,
        process.env.OPT_WASM || "",
        process.env.META_WASM,
        account,
        0,
        0x00
    )
    console.log("Hello Program ID:", program.programId);
}

main();

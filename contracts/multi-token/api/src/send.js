const { GearApi, GearKeyring, getWasmMetadata } = require("@gear-js/api");
const { readFileSync } = require('fs');

require('dotenv').config();

function toHex(str) {
    var result = '';
    for (var i=0; i<str.length; i++) {
      result += str.charCodeAt(i).toString(16);
    }
    return result;
  }

async function main() {
    const gearApi = await GearApi.create();
    const jsonKeyring = readFileSync('./account.json').toString();;
    const account = GearKeyring.fromJson(jsonKeyring, 'Google06!!');
    const metaFile = readFileSync(process.env.META_WASM);
    const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;
    try {
        let somePayload = {
            // Payload goes here
        }

        const gas = await gearApi.program.gasSpent.handle(
            account.publicKey,
            process.env.PROGRAM_ID,
            somePayload,
            0,
            meta,
        );
        console.log("GAS", gas.toHuman());

        const message = {
            destination: process.env.PROGRAM_ID, // programId
            payload: somePayload,
            gasLimit: gas,
            value: 0,
        };
        // In that case payload will be encoded using meta.handle_input type
        await gearApi.message.submit(message, meta);
        // So if you want to use another type you can specify it
        // await gearApi.message.submit(message, meta, meta.async_handle_input); // For example
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
    try {
        await gearApi.message.signAndSend(account, (event) => {
            console.log("EVENT", event.toHuman());
        });
    } catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
}

main();

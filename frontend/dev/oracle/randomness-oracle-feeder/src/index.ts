import {
    GearApi,
    GearKeyring,
    decodeAddress,
    ProgramMetadata,
    getProgramMetadata,
} from "@gear-js/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { HexString } from "@polkadot/util/types";
import * as dotenv from "dotenv";
import { readFileSync } from "fs";
import { Random } from "./types";
import { fetchRandomValue } from "./utils";

dotenv.config();

const ENDPOINT_URL = process.env.ENDPOINT_URL || "ws://127.0.0.1:9944";

const ORACLE_ADDRESS: HexString =
    (process.env.ORACLE_ADDRESS as HexString) || "0x0";
const ORACLE_METADATA: HexString =
    (process.env.ORACLE_METADATA as HexString) || "0x0";

const KEYRING_PASSPHRASE = process.env.KEYRING_PASSPHRASE;
const KEYRING_PATH = process.env.KEYRING_PATH;
const KEYRING_MNEMONIC = process.env.KEYRING_MNEMONIC;
const KEYRING_SEED = process.env.KEYRING_SEED;

const getKeyring = async (): Promise<KeyringPair | undefined> => {
    if (KEYRING_MNEMONIC !== undefined) {
        return await GearKeyring.fromMnemonic(KEYRING_MNEMONIC);
    }

    if (KEYRING_SEED !== undefined) {
        return await GearKeyring.fromSeed(KEYRING_SEED);
    }

    if (KEYRING_PATH !== undefined && KEYRING_PASSPHRASE !== undefined) {
        return GearKeyring.fromJson(
            readFileSync(KEYRING_PATH).toString(),
            KEYRING_PASSPHRASE
        );
    }

    return undefined;
};

const updateOracleValue = async (
    gearApi: GearApi,
    oracleMeta: ProgramMetadata,
    keyring: KeyringPair,
    data: Random
) => {
    try {
        const payload = oracleMeta.createType(4, {
            SetRandomValue: {
                round: data.round,
                value: {
                    randomness: [data.randomness[0], data.randomness[1]],
                    signature: data.signature,
                    prev_signature: data.prevSignature,
                },
            },
        });

        const gas = await gearApi.program.calculateGas.handle(
            decodeAddress(keyring.address),
            ORACLE_ADDRESS,
            payload.toHex(),
            0,
            true,
            oracleMeta
        );

        let extrinsic = gearApi.message.send({
            destination: ORACLE_ADDRESS,
            payload: payload.toHex(),
            gasLimit: gas.min_limit,
            value: 0,
        });

        await extrinsic.signAndSend(keyring, (event: any) => {
            if (event.isError) {
                throw new Error("Can't send tx");
            } else {
                console.log(`[+] UpdateValue(${data})`);
            }
        });
    } catch (error: any) {
        console.log(`[-] Failed to send tx: ${error}`);
    }
};

const main = async () => {
    // 1. Connect to node
    const gearApi = await GearApi.create({
        providerAddress: ENDPOINT_URL,
    });

    console.log(
        `[+] Started with: ${await gearApi.nodeName()}-${await gearApi.nodeVersion()}`
    );

    // 2. Load oracle program metadata
    const oracleMeta = getProgramMetadata(ORACLE_METADATA);

    // 3. Load Keyring from one of provided methods
    const keyring = await getKeyring();
    if (keyring === undefined) {
        console.log("[-] Unable to load keypair by provided methods");
        return;
    }

    // 4. Feed oracle via external API
    setInterval(async () => {
        const data = await fetchRandomValue();
        console.log(`New tick: ${data.round}`);

        await updateOracleValue(gearApi, oracleMeta, keyring, data);
    }, 30000);
};

main();

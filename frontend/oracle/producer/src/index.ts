import {
  GearApi,
  GearKeyring,
  getWasmMetadata,
  CreateType,
  decodeAddress,
  Hex,
} from "@gear-js/api";
import * as dotenv from "dotenv";
import { readFileSync } from "fs";
import { OracleQueueItem, OracleUpdateValue } from "./types";
import { getRandomNumber } from "./utils";

dotenv.config();

const ENDPOINT_URL = process.env.ENDPOINT_URL || "";

const ORACLE_ADDRESS: Hex = (process.env.ORACLE_ADDRESS as Hex) || "0x";
const ORACLE_META_WASM_PATH = process.env.ORACLE_META_WASM_PATH || "";
const ORACLE_META_WASM_BUFFER = readFileSync(ORACLE_META_WASM_PATH);

const KEYRING_PATH = process.env.KEYRING_PATH || "";
const KEYRING_PASSPHRASE = process.env.KEYRING_PASSPHRASE || "";
const KEYRING = GearKeyring.fromJson(
  readFileSync(KEYRING_PATH).toString(),
  KEYRING_PASSPHRASE
);

const getOracleRequestsQueue = async (
  gearApi: GearApi
): Promise<OracleQueueItem[]> => {
  const state = (
    await gearApi.programState.read(
      ORACLE_ADDRESS,
      ORACLE_META_WASM_BUFFER,
      "GetRequestsQueue"
    )
  ).toHuman();

  const oracleQueueItems: OracleQueueItem[] = (state as any).RequestsQueue.map(
    (oracleQueueItem: string[]) => {
      const [id, caller] = oracleQueueItem;

      return {
        id: parseInt(id),
        caller,
      };
    }
  );

  return oracleQueueItems;
};

const updateOracleValue = async (gearApi: GearApi, item: OracleUpdateValue) => {
  try {
    const oracleMeta = await getWasmMetadata(ORACLE_META_WASM_BUFFER);

    const payload = CreateType.create(
      "Action",
      {
        UpdateValue: {
          id: item.id,
          value: item.value,
        },
      },
      oracleMeta
    );

    const gas = await gearApi.program.calculateGas.handle(
      decodeAddress(KEYRING.address),
      ORACLE_ADDRESS,
      payload.toHex(),
      0,
      true,
      oracleMeta
    );

    let extrinsic = gearApi.message.send(
      {
        destination: ORACLE_ADDRESS,
        payload: payload.toHex(),
        gasLimit: gas.min_limit,
        value: 0,
      },
      undefined,
      "String"
    );
    await extrinsic.signAndSend(KEYRING, (event: any) => {
      if (event.isError) {
        throw new Error("Can't send tx");
      } else {
        console.log(`[+] UpdateValue(${item.id}, ${item.value})`);
      }
    });
  } catch (error: any) {
    console.log(`[-] Failed to send tx`);
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

  // 2. Check actual requests queue
  getOracleRequestsQueue(gearApi).then(
    (oracleQueueItems: OracleQueueItem[]) => {
      Promise.all(
        oracleQueueItems.map((item) =>
          updateOracleValue(gearApi, {
            id: item.id,
            value: getRandomNumber(9999999999999),
          })
        )
      );
    }
  );

  // 3. Listen for new oracle requests
  gearApi.blocks.subscribeNewHeads(async () => {
    const oracleQueueItems = await getOracleRequestsQueue(gearApi);
    if (oracleQueueItems.length !== 0) {
      console.log(`[+] New request(s): ${JSON.stringify(oracleQueueItems)}`);

      Promise.all(
        oracleQueueItems.map((item) =>
          updateOracleValue(gearApi, {
            id: item.id,
            value: getRandomNumber(9999999999999),
          })
        )
      );
    }
  });
};

main();

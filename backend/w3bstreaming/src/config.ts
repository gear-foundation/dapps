import { HexString } from "@gear-js/api";
import assert from "assert";
import { config } from "dotenv";

config();

function getEnv(envName: string, default_?: string): string {
  const env = process.env[envName];
  if (env === undefined && default_) {
    return default_;
  }
  assert.notStrictEqual(env, undefined, `${envName} isn't specified`);
  return env as string;
}

export default {
  port: process.env.PORT || 3001,
  wsAddress: getEnv("REACT_APP_NODE_ADDRESS", "wss://testnet.vara-network.io"),
  programId: getEnv("PROGRAM_ID") as HexString,
  dnsApiUrl: getEnv("REACT_APP_DNS_API_URL"),
  dnsName: getEnv("REACT_APP_DNS_NAME"),
};

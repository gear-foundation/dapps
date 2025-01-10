import { GearApi, HexString } from "@gear-js/api";
import { Program } from "./lib";
import config from "../config";

export const api = new GearApi({ providerAddress: config.wsAddress });

export const initProgram = async () => {
  let programId = config.programId;

  try {
    const response = await fetch(
      `${config.dnsApiUrl}/dns/by_name/${config.dnsName}`
    );
    const dns = await response.json();
    if (dns.address) {
      programId = dns.address as HexString;
    }
  } catch (error) {
    const { message } = error as Error;
    console.error(message);
  }

  return new Program(api, programId);
};

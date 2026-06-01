import { GearApi, HexString } from "@gear-js/api";
import { Program } from "./lib";
import config from "../config";
import { SailsProgram } from "../dns/lib";

export const api = new GearApi({ providerAddress: config.wsAddress });

export const initProgram = async () => {
  let programId = config.programId;

  try {
    await api.isReadyOrError;
    const dnsProgram = new SailsProgram(
      api,
      config.dnsContractAddress as `0x${string}`,
    );
    const info = await dnsProgram.dns
      .getContractInfoByName(config.dnsName)
      .call();

    if (info?.program_id) {
      programId = info.program_id as HexString;
    }
  } catch (error) {
    const { message } = error as Error;
    console.error(message);
  }

  return new Program(api, programId);
};

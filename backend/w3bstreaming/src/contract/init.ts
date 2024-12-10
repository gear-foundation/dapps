import { GearApi } from "@gear-js/api";
import { Program } from "./lib";
import config from "../config";

export const api = new GearApi({ providerAddress: config.wsAddress });

export const program = new Program(api, config.programId);

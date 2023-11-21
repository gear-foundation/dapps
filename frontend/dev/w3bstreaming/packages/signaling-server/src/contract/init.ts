import { GearApi, StateMetadata, getStateMetadata } from '@gear-js/api';
import config from '../config';
import { readFileSync } from 'fs';

export const api = new GearApi({ providerAddress: config.wsAddress });

export const stateWasm = readFileSync(config.pathToStateWasm);

export let stateMeta: StateMetadata;

export const isMetaReady = getStateMetadata(stateWasm).then((meta) => {
  stateMeta = meta;
  return true;
});

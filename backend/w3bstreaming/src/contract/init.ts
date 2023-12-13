import { GearApi, ProgramMetadata } from '@gear-js/api';
import config from '../config';
import { readFileSync } from 'fs';

export const api = new GearApi({ providerAddress: config.wsAddress });

export const res = readFileSync(config.pathToMeta, 'utf-8');

export const meta = ProgramMetadata.from(`0x${res}`);

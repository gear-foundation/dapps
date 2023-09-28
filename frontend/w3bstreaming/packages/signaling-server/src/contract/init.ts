import { GearApi, getProgramMetadata } from '@gear-js/api';
import config from '../config';
import { readFileSync } from 'fs';

export const api = new GearApi({ providerAddress: config.wsAddress });

export const res = readFileSync(config.pathToMeta, 'utf-8');

export const meta = getProgramMetadata(res);

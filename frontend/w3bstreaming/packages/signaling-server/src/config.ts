import assert from 'assert';
import { config } from 'dotenv';

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
  wsAddress: getEnv('WS_ADDRESS', 'ws://127.0.0.1:9944'),
  pathToMeta: getEnv(
    'PATH_TO_META',
    '../../target/wasm32-unknown-unknown/release/web3streaming.meta.txt'
  ),
  programId: getEnv('PROGRAM_ID'),
};

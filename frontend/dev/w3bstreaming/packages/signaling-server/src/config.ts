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
  pathToStateWasm: getEnv(
    'PATH_TO_STATE_WASM',
    '../../target/wasm32-unknown-unknown/release/web3streaming_state.meta.wasm',
  ),
  programId: getEnv('PROGRAM_ID'),
};

const assert = require('assert');
const { readFileSync, writeFileSync } = require('fs');
const { getWasmMetadata } = require('@gear-js/api');

const [pathToEnv, ws] = process.argv.slice(2);

const pathToMeta = process.env.PATH_TO_META;
const programId = process.env.PROGRAM_ID;

assert.notStrictEqual(pathToEnv, undefined, 'Path to .env is not specified');
assert.notStrictEqual(pathToMeta, undefined, 'Path to .meta.wasm is not specified');
assert.notStrictEqual(programId, undefined, 'Program id is not specified');

const main = async () => {
  const wasm = readFileSync(pathToMeta);
  const base64 = wasm.toString('base64');

  const { types } = await getWasmMetadata(wasm);

  const env = `REACT_APP_NODE_ADDRESS=${ws || 'wss://rpc-node.gear-tech.io'}\n
REACT_APP_PROGRAM_ID=${programId}\n
REACT_APP_META_TYPES=${types}\n
REACT_APP_META_BUFFER_BASE_64=${base64}`;

  writeFileSync(pathToEnv, env);
};

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });

import { parse } from 'yaml';
import fs from 'fs';
import { GearApi } from '@gear-js/api';
import { waitReady } from '@polkadot/wasm-crypto';
import { fundAccounts, getAccounts } from './accounts';
import { getPrograms, uploadProgram } from './program';
import { getPayload } from './payload';
import { sendMessage } from './message';
import assert from 'assert';
import { IScheme } from './types';
import { getCodes, uploadCode } from './code';
import { validateScheme } from './validate';

const [pathToScheme] = process.argv.slice(2);
assert.notStrictEqual(pathToScheme, undefined, 'Path to scheme is not specified');

const schemeFile = fs.readFileSync(pathToScheme, 'utf-8');
const scheme: IScheme = parse(schemeFile);
const errors = validateScheme(scheme);
if (errors) {
  console.log(`[*] Scheme validations failed`);
  console.log(`ERRORS:`);
  console.log(errors);
  process.exit(1);
}

const main = async () => {
  await waitReady();
  const api = await GearApi.create({ providerAddress: scheme.wsAddress });
  console.log(`[*] Connected to ${await api.chain()}\n`);
  const accounts = getAccounts(scheme.accounts);
  const programs = getPrograms(scheme.programs);
  const codes = getCodes(scheme.codes || []);

  if (scheme.fund_accounts) {
    console.log(`[*] Fund accounts ${JSON.stringify(scheme.fund_accounts)}\n`);
    await fundAccounts(api, accounts, scheme.prefunded_account, scheme.fund_accounts);
  }

  for (const tx of scheme.transactions) {
    const acc = accounts[tx.account];

    if (tx.type === 'upload_code') {
      const code = codes[tx.code];
      console.log(`[*] Upload code ${JSON.stringify(code.name)}`);
      const { codeHash } = await uploadCode(api, acc, code.path_to_wasm);
      console.log();
      code.hash = codeHash;
      continue;
    }

    if (tx.type === 'upload_program') {
      const program = programs[tx.program];
      console.log(`[*] Upload ${program.name}`);
      const { programId, meta } = await uploadProgram(
        api,
        acc,
        program.path_to_wasm,
        program.path_to_meta,
        getPayload(accounts, programs, codes, program.payload),
        program.value,
      );
      console.log();
      program.address = programId;
      program.meta = meta;
      continue;
    }

    if (tx.type === 'send_message') {
      const program = programs[tx.program];
      const payload = getPayload(accounts, programs, codes, tx.payload);
      console.log(`[*] Send message ${JSON.stringify(payload)} to ${program.name}`);
      if (!program.address) {
        throw new Error(`Program ${tx.program} wasn't uploaded`);
      }
      await sendMessage(api, acc, program.address, program.meta, payload, tx.value, tx.increase_gas);
      console.log();
      continue;
    }
  }
};

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });

import { ICode, IProgram } from './types';
import { KeyringPair } from '@polkadot/keyring/types';
import { decodeAddress } from '@gear-js/api';

const ACC_REGEX = /\$account \w+/g;
const PROG_REGEX = /\$program \d+/g;
const CODE_REGEX = /\$code \d+/g;

export function getPayload(
  accounts: Record<string, KeyringPair>,
  programs: Record<number, IProgram>,
  codes: Record<number, ICode>,
  payload: any,
) {
  if (!payload) {
    return undefined;
  }
  let stringPayload = JSON.stringify(payload);

  const matchAcc = stringPayload.match(ACC_REGEX);
  const matchProg = stringPayload.match(PROG_REGEX);
  const matchCode = stringPayload.match(CODE_REGEX);

  if (matchProg) {
    for (const match of matchProg) {
      const program = programs[Number(match.split(' ')[1])].address;
      stringPayload = stringPayload.replace(match, program);
    }
  }
  if (matchAcc) {
    for (const match of matchAcc) {
      const acc = decodeAddress(accounts[match.split(' ')[1]].address);
      stringPayload = stringPayload.replace(match, acc);
    }
  }
  if (matchCode) {
    for (const match of matchCode) {
      const code = codes[Number(match.split(' ')[1])].hash;
      stringPayload = stringPayload.replace(match, code);
    }
  }
  return JSON.parse(stringPayload);
}

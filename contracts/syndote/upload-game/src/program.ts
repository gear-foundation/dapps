import { GearApi, getProgramMetadata, MessageQueued } from '@gear-js/api';
import { isMsgDispatchedSuccessfully } from './findEvents';
import { KeyringPair } from '@polkadot/keyring/types';
import { u8aToHex, isHex } from '@polkadot/util';
import fs from 'fs';
import { IProgram, SchemeProgram } from './types';

export async function uploadProgram(
  api: GearApi,
  account: KeyringPair,
  pathToProgram: string,
  pathToMeta: string,
  initPayload: any,
  value?: number | string,
) {
  const code = fs.readFileSync(pathToProgram);
  const metaHex = fs.readFileSync(pathToMeta, 'utf-8');
  const meta = getProgramMetadata(isHex(metaHex) ? metaHex : `0x${metaHex}`);
  const gas = await api.program.calculateGas.initUpload(
    u8aToHex(account.addressRaw),
    code,
    initPayload,
    value,
    false,
    meta,
    meta.types.init.input,
  );

  console.log(`  [*] Calculated gas: ${gas.min_limit.toHuman()}`);

  const { programId, extrinsic } = api.program.upload(
    { code, value, gasLimit: gas.min_limit, initPayload },
    meta,
    meta.types.init.input,
  );

  console.log(`  [*] Program id: ${programId}`);

  const [blockHash, msgId]: [`0x${string}`, `0x${string}`] = await new Promise((resolve) =>
    extrinsic.signAndSend(account, ({ events, status }) => {
      const meEvent = events.find(({ event: { method } }) => method === 'MessageQueued');
      if (meEvent) {
        if (status.isInBlock) {
          resolve([status.asInBlock.toHex(), (meEvent.event as MessageQueued).data.id.toHex()]);
        }
      }
    }),
  );

  const isSuccess = await isMsgDispatchedSuccessfully(api, msgId, blockHash);

  if (isSuccess) {
    console.log(`  [*] Program initialized successfuly`);
    return { programId, meta };
  }

  throw new Error(`Program initialization failed`);
}

export function getPrograms(programs: Array<SchemeProgram>): Record<number, IProgram> {
  const result: Record<number, IProgram> = {};

  programs.forEach(({ id, ...rest }) => {
    result[id] = rest;
  });

  return result;
}

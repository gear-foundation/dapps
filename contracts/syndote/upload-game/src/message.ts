import { GearApi, MessageQueued, ProgramMetadata } from '@gear-js/api';
import { getReply, isMsgDispatchedSuccessfully } from './findEvents';
import { KeyringPair } from '@polkadot/keyring/types';
import { u8aToHex } from '@polkadot/util';

export async function sendMessage(
  api: GearApi,
  account: KeyringPair,
  programId: `0x${string}`,
  meta: ProgramMetadata,
  payload: any,
  value?: number | string,
  increaseGas?: number,
) {

  const calculatedGas = await api.program.calculateGas.handle(
    u8aToHex(account.addressRaw),
    programId,
    payload,
    value,
    false,
    meta,
    meta.types.handle.input,
  );

  let gas = calculatedGas.min_limit.toBn();

  if (increaseGas) {
    gas = gas.add(gas.muln(increaseGas));
  }

  console.log(`  [*] Calculated gas: ${calculatedGas.min_limit.toHuman()}. Applied gas: ${gas.toString()}`);

  const extrinsic = api.message.send(
    { destination: programId, value, gasLimit: gas, payload },
    meta,
    meta.types.handle.input,
  );

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

  console.log(`  [*] Message id: ${msgId}`);

  const isSuccess = await isMsgDispatchedSuccessfully(api, msgId, blockHash);

  if (!isSuccess) {
    throw new Error(`Message failed`);
  }
  console.log(`  [*] Message dispatched successfuly`);

  const reply = await getReply(api, programId, msgId, blockHash, 10);

  if (!reply) {
    throw new Error(`Reply was not received`);
  }
  console.log(`  [*] Reply message id: ${reply.msgId}`);
  console.log(
    `  [*] Reply payload ${JSON.stringify(meta.createType(meta.types.handle.output!, reply.payload).toJSON())}`,
  );
}

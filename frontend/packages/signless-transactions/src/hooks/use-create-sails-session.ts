import { HexString } from '@gear-js/api';
import { useAccount, useAlert, useApi, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useBatchSignAndSend } from './use-batch-sign-and-send';
import { KeyringPair } from '@polkadot/keyring/types';
import { BaseProgram } from '@/context/types';
import { CreeateSessionOptions, Options, Session, useCreateBaseSession } from './use-create-base-session';

function useCreateSailsSession(programId: HexString, program: BaseProgram) {
  const { isApiReady } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { batchSignAndSend } = useBatchSignAndSend('all');
  const onError = (message: string) => alert.error(message);
  const { getVoucherExtrinsic, signAndSendDeleteSession } = useCreateBaseSession(programId);

  const { prepareTransactionAsync: prepareCreateSession } = usePrepareProgramTransaction({
    program,
    serviceName: 'session',
    functionName: 'createSession',
  });

  const { prepareTransactionAsync: prepareDeleteSession } = usePrepareProgramTransaction({
    program,
    serviceName: 'session',
    functionName: 'deleteSession',
  });

  const createSession = async (
    session: Session,
    voucherValue: number,
    { shouldIssueVoucher, voucherId, pair, ...options }: Options & CreeateSessionOptions,
  ) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account not found');

    const { key, duration, allowedActions } = session;

    const { transaction } = await prepareCreateSession({
      account: { addressOrPair: pair ? pair.address : account.decodedAddress },
      args: [key, duration, allowedActions],
      voucherId,
    });
    const messageExtrinsic = transaction.extrinsic;

    const txs = shouldIssueVoucher
      ? [messageExtrinsic, await getVoucherExtrinsic(session, voucherValue)]
      : [messageExtrinsic];

    batchSignAndSend(txs, { ...options, onError });
  };

  const deleteSession = async (key: HexString, pair: KeyringPair, options: Options) => {
    if (!account) throw new Error('Account not found');

    const { transaction } = await prepareDeleteSession({
      account: { addressOrPair: account.decodedAddress },
      args: [],
    });

    signAndSendDeleteSession(transaction.extrinsic, key, pair, options);
  };

  return { createSession, deleteSession };
}

export { useCreateSailsSession };

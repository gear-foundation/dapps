import { HexString } from '@gear-js/api';
import { useAccount, useApi, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { KeyringPair } from '@polkadot/keyring/types';
import { BaseProgram } from '@/context/types';
import { CreeateSessionOptions, Options, Session, useCreateBaseSession } from './use-create-base-session';

function useCreateSailsSession(programId: HexString, program: BaseProgram) {
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const { signAndSendCreateSession, signAndSendDeleteSession } = useCreateBaseSession(programId);

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
    console.log("🚀 ~ useCreateSailsSession ~ session:", session)
    console.log("🚀 ~ useCreateSailsSession ~ voucherId:", voucherId)
    console.log("🚀 ~ useCreateSailsSession ~ pair:", pair)
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account not found');

    const { key, duration, allowedActions } = session;

    const { transaction } = await prepareCreateSession({
      account: pair ? { addressOrPair: pair.address } : undefined,
      args: [key, duration, allowedActions],
      voucherId,
    });
    const messageExtrinsic = transaction.extrinsic;

    signAndSendCreateSession(messageExtrinsic, session, voucherValue, options, shouldIssueVoucher);
  };

  const deleteSession = async (key: HexString, pair: KeyringPair, options: Options) => {
    const { transaction } = await prepareDeleteSession({ args: [] });
    signAndSendDeleteSession(transaction.extrinsic, key, pair, options);
  };

  return { createSession, deleteSession };
}

export { useCreateSailsSession };

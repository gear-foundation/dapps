import { HexString, ProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';

import { useBatchSignAndSend } from './use-batch-sign-and-send';

type Session = {
  key: HexString;
  duration: number;
  allowedActions: string[];
};

type Options = {
  onSuccess: () => void;
  onFinally: () => void;
};

function useCreateSession(programId: HexString, metadata: ProgramMetadata | undefined) {
  const { api, isApiReady } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { batchSignAndSend } = useBatchSignAndSend('all');

  const onError = (message: string) => alert.error(message);

  const getMessage = (payload: AnyJson) => {
    const destination = programId;
    // TODO: replace with calculation after release fix
    const gasLimit = 10000000000;

    return { destination, payload, gasLimit };
  };

  const deleteSession = async () => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const message = getMessage({ DeleteSessionFromAccount: null });
    const extrinsic = api.message.send(message, metadata);
    // const voucher = api.voucher.revoke(session.key, programId);

    const txs = [extrinsic];

    batchSignAndSend(txs, { onError });
  };

  const createSession = async (session: Session, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const getVoucher = async (voucherId: `${string}` | undefined) => {
      if (voucherId) {
        const extrinsic = await api.voucher.revoke(session.key, voucherId);
        return {
          extrinsic,
          voucherId,
        };
      }

      const voucher = await api.voucher.issue(session.key, voucherValue, session.duration, [programId]);
      return voucher;
    };

    const message = getMessage({ CreateSession: session });

    const extrinsic = api.message.send(message, metadata);

    const vouchersForAccount = await api.voucher.getAllForAccount(account?.decodedAddress);

    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    const voucher = await getVoucher(accountVoucherId);

    const txs = [extrinsic, voucher.extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  return { createSession, deleteSession };
}

export { useCreateSession };

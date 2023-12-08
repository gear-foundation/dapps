import { HexString } from '@gear-js/api';
import { useAlert, useApi, useHandleCalculateGas } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';

import { useProgramMetadata } from '@dapps-frontend/hooks';

import { ADDRESS } from '@/app/consts';
import metaTxt from '@/features/game/assets/meta/battleship.meta.txt';

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

function useCreateSession() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const metadata = useProgramMetadata(metaTxt);
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, metadata);

  const { batchSignAndSend } = useBatchSignAndSend('all');

  const onError = (message: string) => alert.error(message);

  const getMessage = async (payload: AnyJson) => {
    const destination = ADDRESS.GAME;
    const gasLimit = (await calculateGas(payload)).min_limit;

    return { destination, payload, gasLimit };
  };

  const deleteSession = async () => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const message = await getMessage({ DeleteSessionFromAccount: null });
    const extrinsic = api.message.send(message, metadata);
    // const voucher = api.voucher.revoke(session.key, ADDRESS.GAME);

    const txs = [extrinsic];

    batchSignAndSend(txs, { onError });
  };

  const createSession = async (session: Session, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const message = await getMessage({ CreateSession: session });

    const extrinsic = api.message.send(message, metadata);
    const voucher = api.voucher.issue(session.key, ADDRESS.GAME, voucherValue);

    const txs = [extrinsic, voucher.extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  return { createSession, deleteSession };
}

export { useCreateSession };

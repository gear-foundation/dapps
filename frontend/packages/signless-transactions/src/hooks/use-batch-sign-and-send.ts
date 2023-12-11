import { useApi, useAccount } from '@gear-js/react-hooks';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { web3FromSource } from '@polkadot/extension-dapp';
import { ISubmittableResult } from '@polkadot/types/types';

import { useGetExtrinsicFailedError } from './use-get-extrinsic-failed-error';

type Options = Partial<{
  onSuccess: () => void;
  onError: (error: string) => void;
  onFinally: () => void;
}>;

function useBatchSignAndSend(type?: 'all' | 'force') {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();
  const { getExtrinsicFailedError } = useGetExtrinsicFailedError();

  const getBatch = () => {
    if (!isApiReady) throw new Error('API is not initialized');

    switch (type) {
      case 'all':
        return api.tx.utility.batchAll;

      case 'force':
        return api.tx.utility.forceBatch;

      default:
        return api.tx.utility.batch;
    }
  };

  const handleStatus = (
    { status, events }: ISubmittableResult,
    { onSuccess = () => {}, onError = () => {}, onFinally = () => {} }: Options = {},
  ) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!status.isInBlock) return;

    events
      .filter(({ event }) => event.section === 'system')
      .forEach(({ event }) => {
        const { method } = event;

        if (method === 'ExtrinsicSuccess' || method === 'ExtrinsicFailed') onFinally();

        if (method === 'ExtrinsicSuccess') return onSuccess();

        if (method === 'ExtrinsicFailed') {
          const message = getExtrinsicFailedError(event);

          onError(message);
          console.error(message);
        }
      });
  };

  const batchSignAndSend = async (
    txs: SubmittableExtrinsic<'promise', ISubmittableResult>[],
    options: Options = {},
  ) => {
    if (!account) throw new Error('No account address');

    const { address, meta } = account;
    const batch = getBatch();
    const statusCallback = (result: ISubmittableResult) => handleStatus(result, options);

    web3FromSource(meta.source)
      .then(({ signer }) => batch(txs).signAndSend(address, { signer }, statusCallback))
      .catch(({ message }: Error) => {
        const { onError = () => {}, onFinally = () => {} } = options;

        onError(message);
        onFinally();
      });
  };

  return { batchSignAndSend };
}

export { useBatchSignAndSend };

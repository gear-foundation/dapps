import { useAlert } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';

import { useCheckBalance } from '@dapps-frontend/hooks';

import { usePending } from './';

export type Options = {
  onSuccess?: () => void;
  onError?: (error?: Error) => void;
};

export const useSignAndSend = () => {
  const { checkBalance } = useCheckBalance();
  const { setPending } = usePending();
  const alert = useAlert();

  const signAndSend = async (
    transaction: TransactionReturn<() => GenericTransactionReturn<null>>,
    options?: Options,
  ) => {
    const { onSuccess, onError } = options || {};
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());
    checkBalance(
      calculatedGas,
      async () => {
        try {
          const { response } = await transaction.signAndSend();
          await response();
          onSuccess?.();
          setPending(false);
        } catch (e) {
          onError?.(e as Error);
          setPending(false);
          console.error(e);
          if (typeof e === 'string') {
            alert.error(e);
          }
        }
      },
      onError,
    );
  };

  return { signAndSend };
};

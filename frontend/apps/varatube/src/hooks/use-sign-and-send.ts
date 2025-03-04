import { useAlert } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';

import { useCheckBalance } from '@dapps-frontend/hooks';

export type Options<T = null> = {
  onSuccess?: (result: T) => void;
  onError?: () => void;
};

export const useSignAndSend = () => {
  const { checkBalance } = useCheckBalance();
  const alert = useAlert();

  const signAndSend = async <T = null>(
    transaction: TransactionReturn<() => GenericTransactionReturn<T>>,
    options?: Options<T>,
  ) => {
    const { onSuccess, onError } = options || {};
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());
    checkBalance(
      calculatedGas,
      async () => {
        try {
          const { response } = await transaction.signAndSend();
          const result = await response();
          onSuccess?.(result);
        } catch (e) {
          onError?.();
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

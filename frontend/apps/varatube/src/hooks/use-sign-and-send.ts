import { useAlert } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';

import { useCheckBalance } from '@dapps-frontend/hooks';
import { getErrorMessage } from '@dapps-frontend/ui';

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
        } catch (error) {
          onError?.();
          console.error(error);
          alert.error(getErrorMessage(error));
        }
      },
      onError,
    );
  };

  return { signAndSend };
};

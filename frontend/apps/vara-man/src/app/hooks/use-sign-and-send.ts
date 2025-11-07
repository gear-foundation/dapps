import { useAlert } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';
import { useEzTransactions } from 'gear-ez-transactions';

import { useCheckBalance } from '@dapps-frontend/hooks';
import { getErrorMessage } from '@dapps-frontend/ui';

export type Options = {
  onSuccess?: () => void;
  onError?: () => void;
};

export const useSignAndSend = () => {
  const { gasless, signless } = useEzTransactions();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const alert = useAlert();

  const signAndSend = (
    transaction: TransactionReturn<() => GenericTransactionReturn<null>>,
    { onSuccess, onError }: Options,
  ) => {
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());

    const executeTransaction = async () => {
      try {
        const { response } = await transaction.signAndSend();
        await response();
        onSuccess?.();
      } catch (error) {
        onError?.();
        console.error(error);
        alert.error(getErrorMessage(error));
      }
    };

    checkBalance(
      calculatedGas,
      () => {
        void executeTransaction();
      },
      onError,
    );
  };

  return { signAndSend };
};

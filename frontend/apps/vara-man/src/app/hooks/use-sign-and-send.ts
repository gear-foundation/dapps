import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useAlert } from '@gear-js/react-hooks';
import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/esm/hooks/sails/types';

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

  const signAndSend = async (
    transaction: TransactionReturn<() => GenericTransactionReturn<null>>,
    { onSuccess, onError }: Options,
  ) => {
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());
    checkBalance(
      calculatedGas,
      async () => {
        try {
          const { response } = await transaction.signAndSend();
          await response();
          onSuccess?.();
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

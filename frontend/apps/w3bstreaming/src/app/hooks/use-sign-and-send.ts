import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';

import { useCheckBalance } from '@dapps-frontend/hooks';

export const useSignAndSend = () => {
  const { checkBalance } = useCheckBalance();

  const signAndSend = async (transaction: TransactionReturn<() => GenericTransactionReturn<null>>) => {
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());

    return new Promise<void>((resolve, reject) => {
      checkBalance(
        calculatedGas,
        async () => {
          try {
            const { response } = await transaction.signAndSend();
            await response();
            resolve();
          } catch (e) {
            reject(e);
          }
        },
        () => reject(),
      );
    });
  };

  return { signAndSend };
};

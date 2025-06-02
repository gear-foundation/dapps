import { GenericTransactionReturn, TransactionReturn } from '@gear-js/react-hooks/dist/hooks/sails/types';

import { useCheckBalanceAsync } from '@dapps-frontend/hooks';

export const useSignAndSend = () => {
  const { checkBalance } = useCheckBalanceAsync();

  const signAndSend = async (transaction: TransactionReturn<() => GenericTransactionReturn<null>>) => {
    const calculatedGas = Number(transaction.extrinsic.args[2].toString());
    await checkBalance(calculatedGas);

    const { response } = await transaction.signAndSend();
    await response();
  };

  return { signAndSend };
};

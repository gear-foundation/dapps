import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

type Params = {
  partialDecs: PartialDec[];
};

export const useSubmitPartialDecryptionsMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'submitPartialDecryptions',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ partialDecs }: Params) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams({ isAutoSignlessEnabled: true });
    const result = await sendTransactionAsync({
      args: [partialDecs, sessionForAccount],
      ...params,
      gasLimit: { increaseGas: 30 },
    });
    return result;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { mutateAsync, isPending };
};

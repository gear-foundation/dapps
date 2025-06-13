import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

type Params = {
  instances: Array<VerificationVariables>;
};

export const useSubmitTablePartialDecryptionsMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'submitTablePartialDecryptions',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ instances }: Params) => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [instances], ...params, gasLimit: { increaseGas: 30 } });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { submitTablePartialDecryptionsMessage: mutateAsync, isPending };
};

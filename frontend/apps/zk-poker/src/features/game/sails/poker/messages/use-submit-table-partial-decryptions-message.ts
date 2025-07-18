import { useAlert, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';
import { useAutoSignless } from '@/features/signless';

type Params = {
  instances: Array<VerificationVariables>;
};

export const useSubmitTablePartialDecryptionsMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { executeWithSessionModal } = useAutoSignless();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'submitTablePartialDecryptions',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ instances }: Params) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [instances, sessionForAccount],
      ...params,
      gasLimit: { increaseGas: 30 },
    });

    await executeWithSessionModal(transaction, sessionForAccount);
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { submitTablePartialDecryptionsMessage: mutateAsync, isPending };
};

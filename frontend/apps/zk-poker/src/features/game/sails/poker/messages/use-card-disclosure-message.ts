import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePokerProgram } from '@/app/utils';

type Params = {
  instances: Array<[Card, VerificationVariables]>;
};

export const useCardDisclosureMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'cardDisclosure',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ instances }: Params) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams({ isAutoSignlessEnabled: true });
    const result = await sendTransactionAsync({ args: [instances, sessionForAccount], ...params });
    return result;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { cardDisclosureMessage: mutateAsync, isPending };
};

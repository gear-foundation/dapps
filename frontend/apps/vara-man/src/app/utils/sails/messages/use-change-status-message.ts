import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { Status, useProgram } from '@/app/utils';

export const useChangeStatusMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'changeStatus',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const changeStatusMessage = async (status: Status, options: Options) => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [status],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { changeStatusMessage };
};

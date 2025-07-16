import { useAlert, useSendProgramTransaction, usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { usePtsProgram } from '@/app/utils';

export const useGetAccuralMessage = () => {
  const program = usePtsProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'pts',
    functionName: 'getAccural',
  });

  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'pts',
    functionName: 'getAccural',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async () => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const tx1 = await prepareTransactionAsync({ args: [], ...params });

    // ! TODO: remove this
    console.log('🚀 ~ tx ~ transaction.extrinsic.toHex():', tx1.transaction.extrinsic.toHex());

    const result = await sendTransactionAsync({ args: [], ...params });

    return result.awaited;
  };

  const { mutateAsync: getAccuralMessage, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
    onSuccess: () => alert.success('PTS claimed'),
  });

  return { getAccuralMessage, isPending };
};

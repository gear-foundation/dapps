import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-make-transaction';

export const useCancelGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'cancelGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const cancelGameMessage = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({ args: [sessionForAccount], ...params });

    return transaction;
  };

  return { cancelGameMessage };
};

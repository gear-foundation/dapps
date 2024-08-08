import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-prepare-ez-transaction-params';

export const useLeaveGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'leaveGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const leaveGameMessage = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({ args: [sessionForAccount], ...params });
    return transaction;
  };

  return { leaveGameMessage };
};

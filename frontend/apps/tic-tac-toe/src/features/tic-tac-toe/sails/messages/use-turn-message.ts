import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { useProgram } from '@/app/utils';

export const useTurnMessage = () => {
  const program = useProgram();
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams({
    isAutoSignlessEnabled: true,
    autoSignless: { allowedActions: SIGNLESS_ALLOWED_ACTIONS },
  });
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'turn',
  });

  const turnMessage = async (step: number) => {
    const params = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({ args: [step, params.sessionForAccount], ...params });

    await transaction.signAndSend();
  };

  return { turnMessage };
};

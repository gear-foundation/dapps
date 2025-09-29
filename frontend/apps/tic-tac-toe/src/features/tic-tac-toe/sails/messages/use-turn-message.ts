import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useAutoSignless, type PrepareEzTransactionParamsResult } from 'gear-ez-transactions';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { useProgram } from '@/app/utils';

export const useTurnMessage = () => {
  const program = useProgram();
  const { executeWithSessionModal } = useAutoSignless({
    allowedActions: SIGNLESS_ALLOWED_ACTIONS,
  });
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'turn',
  });

  const turnMessage = async (step: number) =>
    executeWithSessionModal(({ sessionForAccount, ...params }: PrepareEzTransactionParamsResult) =>
      prepareTransactionAsync({ args: [step, sessionForAccount], ...params }),
    );

  return { turnMessage };
};

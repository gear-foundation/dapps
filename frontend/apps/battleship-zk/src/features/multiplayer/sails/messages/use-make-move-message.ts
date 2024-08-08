import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { usePrepareEzTransactionParams } from '@/app/utils/use-prepare-ez-transaction-params';

export const useMakeMoveMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'makeMove',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const makeMoveMessage = async (
    step: number | null,
    verify_variables: VerificationVariables | null,
    game_id?: string,
  ) => {
    if (!game_id) throw new Error('game_id does not found');
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [game_id, verify_variables, step, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { makeMoveMessage };
};

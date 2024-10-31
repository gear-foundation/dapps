import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils/sails';

export const useJoinGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'joinGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const joinGameMessage = async (game_id: string, name: string, value: bigint) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams(true);
    const { transaction } = await prepareTransactionAsync({
      args: [game_id, name, sessionForAccount],
      ...params,
      value,
    });
    return transaction;
  };

  return { joinGameMessage };
};

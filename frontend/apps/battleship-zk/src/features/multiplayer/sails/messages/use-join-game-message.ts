import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-make-transaction';

export const useJoinGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'joinGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const joinGameMessage = async (game_id: string, name: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [game_id, name, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { joinGameMessage };
};

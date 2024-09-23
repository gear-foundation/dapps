import { useSendProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils';

export const useStartGameMessage = () => {
  const program = useProgram();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'ticTacToe',
    functionName: 'startGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const startGameMessage = async () => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { result } = await sendTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });
    await result.response();
    return;
  };

  return { startGameMessage };
};

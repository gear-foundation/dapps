import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { Level, useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useFinishSingleGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'finishSingleGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const finishSingleGameMessage = async (
    goldCoins: number,
    silverCoins: number,
    level: Level,
    options: Options,
  ) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [goldCoins, silverCoins, level, sessionForAccount],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { finishSingleGameMessage };
};

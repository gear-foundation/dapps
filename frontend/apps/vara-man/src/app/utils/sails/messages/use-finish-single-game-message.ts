import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { useProgram } from '@/app/utils';

export const useFinishSingleGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'finishSingleGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const finishSingleGameMessage = async (goldCoins: number, silverCoins: number, level: Level, options: Options) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [goldCoins, silverCoins, level, sessionForAccount],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { finishSingleGameMessage };
};

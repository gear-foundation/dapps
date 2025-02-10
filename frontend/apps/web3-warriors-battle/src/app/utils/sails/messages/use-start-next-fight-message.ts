import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { useProgram } from '@/app/utils';
import { usePending } from '@/features/game/hooks';

export const useStartNextFightMessage = () => {
  const program = useProgram();

  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'startNextFight',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();
  const { setPending } = usePending();

  const startNextFightMessage = async (options?: Options) => {
    setPending(true);

    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });

    signAndSend(transaction, options);
  };

  return { startNextFightMessage };
};

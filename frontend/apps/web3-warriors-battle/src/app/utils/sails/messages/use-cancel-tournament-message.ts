import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { usePending } from '@/features/game/hooks';

export const useCancelTournamentMessage = () => {
  const program = useProgram();

  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'cancelTournament',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();
  const { setPending } = usePending();

  const cancelTournamentMessage = async (options: Options) => {
    setPending(true);

    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });

    signAndSend(transaction, options);
  };

  return { cancelTournamentMessage };
};

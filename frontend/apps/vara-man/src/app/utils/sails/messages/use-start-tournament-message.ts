import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useStartTournamentMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'startTournament',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const startTournamentMessage = async (options: Options) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [sessionForAccount],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { startTournamentMessage };
};

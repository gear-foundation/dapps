import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { Level, useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useCreateNewTournamentMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'createNewTournament',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const createNewTournamentMessage = async (
    value: bigint,
    tournamentName: string,
    name: string,
    level: Level,
    durationMs: number,
    options: Options,
  ) => {
    const isSendFromBaseAccount = value !== 0n;
    const { sessionForAccount, ...params } = await prepareEzTransactionParams(isSendFromBaseAccount);
    const { transaction } = await prepareTransactionAsync({
      args: [tournamentName, name, level, durationMs, sessionForAccount],
      ...params,
      value,
    });
    signAndSend(transaction, options);
  };

  return { createNewTournamentMessage };
};

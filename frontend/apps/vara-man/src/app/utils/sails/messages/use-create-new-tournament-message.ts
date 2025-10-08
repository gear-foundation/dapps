import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { Level, useProgram } from '@/app/utils';

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
    const sendFromBaseAccount = value !== 0n;
    const { sessionForAccount, ...params } = await prepareEzTransactionParams({ sendFromBaseAccount });
    const { transaction } = await prepareTransactionAsync({
      args: [tournamentName, name, level, durationMs, sessionForAccount],
      ...params,
      value,
    });
    signAndSend(transaction, options);
  };

  return { createNewTournamentMessage };
};

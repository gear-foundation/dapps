import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useRecordTournamentResultMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'recordTournamentResult',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const recordTournamentResultMessage = async (
    time: number,
    goldCoins: number,
    silverCoins: number,
    options: Options,
  ) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [time, goldCoins, silverCoins, sessionForAccount],
      ...params,
    });
    signAndSend(transaction, options);
  };

  return { recordTournamentResultMessage };
};

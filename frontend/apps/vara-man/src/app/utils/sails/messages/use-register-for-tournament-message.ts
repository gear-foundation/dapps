import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { useProgram } from '@/app/utils';
import { useSignAndSend, Options } from '@/app/hooks/use-sign-and-send';

export const useRegisterForTournamentMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varaMan',
    functionName: 'registerForTournament',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const registerForTournamentMessage = async (value: bigint, adminId: string, name: string, options: Options) => {
    const isSendFromBaseAccount = value !== 0n;
    const { sessionForAccount, ...params } = await prepareEzTransactionParams(isSendFromBaseAccount);
    const { transaction } = await prepareTransactionAsync({
      args: [adminId, name, sessionForAccount],
      ...params,
      value,
    });
    signAndSend(transaction, options);
  };

  return { registerForTournamentMessage };
};

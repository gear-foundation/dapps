import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
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
  const { signAndSend } = useSignAndSend();
  const { setPending } = usePending();

  const cancelTournamentMessage = async (options: Options) => {
    setPending(true);
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { cancelTournamentMessage };
};

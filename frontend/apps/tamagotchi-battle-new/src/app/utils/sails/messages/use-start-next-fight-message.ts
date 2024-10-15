import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { usePending } from '@/features/game/hooks';

export const useStartNextFightMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'startNextFight',
  });
  const { signAndSend } = useSignAndSend();
  const { setPending } = usePending();

  const startNextFightMessage = async (options?: Options) => {
    setPending(true);
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { startNextFightMessage };
};

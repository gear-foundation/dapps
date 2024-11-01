import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { Move, useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

export const useMakeMoveMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'makeMove',
  });
  const { signAndSend } = useSignAndSend();

  const makeMoveMessage = async (move: Move, options?: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [move],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { makeMoveMessage };
};

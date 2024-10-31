import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Options, useSignAndSend } from 'app/hooks';

type Params = {
  fuel: number;
  payload: number;
};

export const useStartGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'startGame',
  });
  const { signAndSend } = useSignAndSend();

  const startGameMessage = async ({ fuel, payload }: Params, options?: Options) => {
    try {
      const { transaction } = await prepareTransactionAsync({
        args: [fuel, payload],
        gasLimit: { increaseGas: 20 },
      });
      signAndSend(transaction, options);
    } catch (error) {
      console.error(error);
      options?.onError?.(error as Error);
    }
  };

  return { startGameMessage };
};

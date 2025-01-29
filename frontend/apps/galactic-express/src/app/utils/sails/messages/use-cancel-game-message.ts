import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks';

export const useCancelGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'cancelGame',
  });
  const { signAndSend } = useSignAndSend();

  const cancelGameMessage = async (options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { cancelGameMessage };
};

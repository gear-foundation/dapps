import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from 'app/utils';
import { Options, useSignAndSend } from 'app/hooks';

export const useCancelRegisterMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'cancelRegister',
  });
  const { signAndSend } = useSignAndSend();

  const cancelRegisterMessage = async (options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 10 },
    });
    signAndSend(transaction, options);
  };

  return { cancelRegisterMessage };
};

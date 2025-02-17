import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useSignAndSend } from '@/app/hooks';
import { Participant, useProgram } from '@/app/utils';

type Params = {
  creator: HexString;
  participant: Participant;
  value?: bigint;
};

export const useRegisterMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'register',
  });
  const { signAndSend } = useSignAndSend();

  const registerMessage = async ({ value, creator, participant }: Params, options?: Options) => {
    try {
      const { transaction } = await prepareTransactionAsync({
        args: [creator, participant],
        gasLimit: { increaseGas: 10 },
        value,
      });
      signAndSend(transaction, options);
    } catch (error) {
      console.error(error);
      options?.onError?.(error as Error);
    }
  };

  return { registerMessage };
};

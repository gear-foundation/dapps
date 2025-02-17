import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useSignAndSend } from '@/app/hooks';
import { useProgram } from '@/app/utils';

type Params = {
  name: string;
  value?: bigint;
};

export const useCreateNewSessionMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'galacticExpress',
    functionName: 'createNewSession',
  });
  const { signAndSend } = useSignAndSend();

  const createNewSessionMessage = async ({ value, name }: Params, options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [name],
      gasLimit: { increaseGas: 10 },
      value,
    });
    signAndSend(transaction, options);
  };

  return { createNewSessionMessage };
};

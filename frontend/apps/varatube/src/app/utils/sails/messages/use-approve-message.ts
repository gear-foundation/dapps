import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { Options, useSignAndSend } from '@/hooks/use-sign-and-send';
import { useVftProgram } from '../sails';

type Params = {
  spender: string;
  value: number | string | bigint;
};

export const useApproveMessage = () => {
  const program = useVftProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'vft',
    functionName: 'approve',
  });
  const { signAndSend } = useSignAndSend();

  const approveMessage = async ({ spender, value }: Params, options: Options<boolean>) => {
    const { transaction } = await prepareTransactionAsync({
      args: [spender, value],

      gasLimit: { increaseGas: 20 },
    });
    console.log('SIGN APPROOVE');
    signAndSend(transaction, options);
  };

  return { approveMessage };
};

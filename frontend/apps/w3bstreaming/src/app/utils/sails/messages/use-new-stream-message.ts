import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';

type Params = {
  title: string;
  description: string | null;
  startTime: number | string | bigint;
  endTime: number | string | bigint;
  imgLink: string;
};

export const useNewStreamMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'w3Bstreaming',
    functionName: 'newStream',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const newStreamMessage = async ({ title, description, startTime, endTime, imgLink }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [title, description, startTime, endTime, imgLink],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { newStreamMessage };
};

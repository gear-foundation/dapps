import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { Options, useExecuteWithPending, useSignAndSend } from '@/app/hooks';

type Params = {
  name: string | null;
  surname: string | null;
  img_link: string | null;
  time_zone: string | null;
};

export const useEditProfileMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'w3Bstreaming',
    functionName: 'editProfile',
  });
  const { signAndSend } = useSignAndSend();
  const { executeWithPending } = useExecuteWithPending();

  const editProfileMessage = async ({ name, surname, img_link, time_zone }: Params, options?: Options) =>
    executeWithPending(async () => {
      const { transaction } = await prepareTransactionAsync({
        args: [name, surname, img_link, time_zone],
        gasLimit: { increaseGas: 10 },
      });
      await signAndSend(transaction);
    }, options);

  return { editProfileMessage };
};

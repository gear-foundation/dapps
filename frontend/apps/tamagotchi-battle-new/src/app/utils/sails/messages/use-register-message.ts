import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { Appearance, useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

type RegisterParams = {
  value: bigint;
  gameId: `0x${string}`;
  name: string;
  warriorId: string | null;
  appearance: Appearance | null;
  attack: number;
  defence: number;
  dodge: number;
};

export const useRegisterMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'register',
  });
  const { signAndSend } = useSignAndSend();

  const registerMessage = async (params: RegisterParams, options: Options) => {
    const { value, gameId, name, warriorId, appearance, attack, defence, dodge } = params;
    const { transaction } = await prepareTransactionAsync({
      args: [gameId, warriorId, appearance, name, attack, defence, dodge],
      gasLimit: { increaseGas: 10 },
      value,
    });
    signAndSend(transaction, options);
  };

  return { registerMessage };
};

import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { Appearance, useProgram } from '@/app/utils';

type RegisterParams = {
  value: bigint;
  gameId: HexString;
  name: string;
  warriorId: HexString | null;
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

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const registerMessage = async (
    { value, gameId, name, warriorId, appearance, attack, defence, dodge }: RegisterParams,
    options: Options,
  ) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [gameId, warriorId, appearance, name, attack, defence, dodge, sessionForAccount],
      value,
      ...params,
    });

    signAndSend(transaction, options);
  };

  return { registerMessage };
};

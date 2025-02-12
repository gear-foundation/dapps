import { HexString } from '@gear-js/api';
import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';

import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';
import { Appearance, useProgram } from '@/app/utils';

type CreateNewBattleParams = {
  value: bigint;
  name: string;
  tournamentName: string;
  warriorId: HexString | null;
  appearance: Appearance | null;
  attack: number;
  defence: number;
  dodge: number;
};

export const useCreateNewBattleMessage = () => {
  const program = useProgram();

  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'battle',
    functionName: 'createNewBattle',
  });

  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { signAndSend } = useSignAndSend();

  const createNewBattleMessage = async (
    { value, tournamentName, name, warriorId, appearance, attack, defence, dodge }: CreateNewBattleParams,
    options: Options,
  ) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();

    const { transaction } = await prepareTransactionAsync({
      args: [tournamentName, name, warriorId, appearance, attack, defence, dodge, sessionForAccount],
      value,
      ...params,
    });

    signAndSend(transaction, options);
  };

  return { createNewBattleMessage };
};

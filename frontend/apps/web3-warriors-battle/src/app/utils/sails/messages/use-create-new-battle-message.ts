import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { Appearance, useProgram } from '@/app/utils';
import { Options, useSignAndSend } from '@/app/hooks/use-sign-and-send';

type CreateNewBattleParams = {
  value: bigint;
  name: string;
  tournamentName: string;
  warriorId: string | null;
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
  const { signAndSend } = useSignAndSend();

  const createNewBattleMessage = async (params: CreateNewBattleParams, options: Options) => {
    const { value, tournamentName, name, warriorId, appearance, attack, defence, dodge } = params;
    const { transaction } = await prepareTransactionAsync({
      args: [tournamentName, name, warriorId, appearance, attack, defence, dodge],
      gasLimit: { increaseGas: 10 },
      value,
    });
    signAndSend(transaction, options);
  };

  return { createNewBattleMessage };
};

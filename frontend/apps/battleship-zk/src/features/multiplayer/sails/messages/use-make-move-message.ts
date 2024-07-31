import { useProgram } from '@/app/utils/sails';
import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useMakeMoveMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const makeMoveMessage = async (
    step: number | null,
    verify_variables: VerificationVariables | null,
    game_id?: string,
  ) => {
    if (!game_id) throw new Error('game_id does not found');
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.makeMove(game_id, verify_variables, step, null));

    return await transaction.withGas(gasLimit);
  };

  return { makeMoveMessage };
};

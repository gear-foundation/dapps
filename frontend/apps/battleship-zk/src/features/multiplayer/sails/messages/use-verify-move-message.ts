import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicMoveInput } from '@/features/game/assets/lib/lib';

export const useVerifyMoveMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const verifyMove = async (proof: ProofBytes, public_input: PublicMoveInput, game_id: string) => {
    const transaction = await makeTransaction(program.multiple.verifyMove(proof, public_input, null, game_id));

    return await transaction.withGas(gasLimit);
  };

  return { verifyMove };
};

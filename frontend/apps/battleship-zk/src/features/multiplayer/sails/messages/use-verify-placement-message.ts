import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicStartInput } from '@/app/utils/sails/lib/lib';

export const useVerifyPlacementMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const verifyPlacementMessage = async (proof: ProofBytes, public_input: PublicStartInput, game_id: string) => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.multiple.verifyPlacement(proof, public_input, null, game_id));

    return await transaction.withGas(gasLimit);
  };

  return { verifyPlacementMessage };
};

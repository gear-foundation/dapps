import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicStartInput } from '@/features/game/assets/lib/lib';

export const useVerifyPlacementMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const verifyPlacementMessage = async (proof: ProofBytes, public_input: PublicStartInput, game_id: string) => {
    const transaction = await makeTransaction(program.multiple.verifyPlacement(proof, public_input, null, game_id));

    return await transaction.withGas(gasLimit);
  };

  return { verifyPlacementMessage };
};

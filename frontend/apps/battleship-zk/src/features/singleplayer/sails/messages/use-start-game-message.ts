import { useProgram } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicStartInput } from '@/app/utils/sails/lib/lib';

export const useStartGameMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const startGameMessage = async (proof: ProofBytes, public_input: PublicStartInput) => {
    if (!program) throw new Error('program does not found');

    const transaction = await makeTransaction(program.single.startSingleGame(proof, public_input, null));

    return await transaction.withGas(gasLimit);
  };

  return { startGameMessage };
};

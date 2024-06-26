import { program } from '@/app/utils/sails';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicMoveInput } from '@/app/utils/sails/lib/lib';

export const useVerifyMoveMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();

  const verifyMoveMessage = async (proof: ProofBytes, public_input: PublicMoveInput) => {
    const transaction = await makeTransaction(program.single.verifyMove(proof, public_input, null));

    return await transaction.withGas(gasLimit);
  };

  return { verifyMoveMessage };
};

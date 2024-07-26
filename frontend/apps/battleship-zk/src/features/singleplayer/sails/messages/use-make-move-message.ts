import { useProgram } from '@/app/utils/sails';
import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { useMakeTransaction } from '@/app/utils/use-make-transaction';

export const useMakeMoveMessage = () => {
  const gasLimit = 250_000_000_000n;
  const makeTransaction = useMakeTransaction();
  const program = useProgram();

  const makeMoveMessage = async (step: number | null, verificationVariables: VerificationVariables | null) => {
    const transaction = await makeTransaction(program.single.makeMove(step, verificationVariables, null));

    return await transaction.withGas(gasLimit);
  };

  return { makeMoveMessage };
};

import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-make-transaction';
import { ProofBytes, PublicStartInput } from '@/app/utils/sails/lib/lib';

export const useVerifyPlacementMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'verifyPlacement',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const verifyPlacementMessage = async (proof: ProofBytes, public_input: PublicStartInput, game_id: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [proof, public_input, sessionForAccount, game_id],
      ...params,
    });
    return transaction;
  };

  return { verifyPlacementMessage };
};

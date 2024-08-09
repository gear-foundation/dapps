import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useProgram } from '@/app/utils/sails';
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

import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils/sails';
import { usePrepareEzTransactionParams } from '@/app/utils/use-prepare-ez-transaction-params';
import { ProofBytes, PublicStartInput } from '@/app/utils/sails/lib/lib';

export const useStartGameMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'single',
    functionName: 'startSingleGame',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const startGameMessage = async (proof: ProofBytes, public_input: PublicStartInput) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [proof, public_input, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { startGameMessage };
};

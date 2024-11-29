import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from '@dapps-frontend/ez-transactions';
import { useConfigurationQuery, useProgram } from '@/app/utils/sails';
import { ProofBytes, PublicStartInput } from '@/app/utils/sails/lib/lib';

export const useVerifyPlacementMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'multiple',
    functionName: 'verifyPlacement',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();
  const { data: config } = useConfigurationQuery();

  const verifyPlacementMessage = async (proof: ProofBytes, public_input: PublicStartInput, game_id: string) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [proof, public_input, sessionForAccount, game_id],
      ...params,
    });
    const calculatedGas = BigInt(transaction.extrinsic.args[2].toString());

    // When calculating gas for two players simultaneously,
    // make sure to account for the gas_for_check_time allocated in the contract for a delayed message,
    // which will be deducted from the last signing player.
    const requiredGas = calculatedGas + BigInt(config?.gas_for_check_time || 0);

    await transaction.withGas(requiredGas);
    return transaction;
  };

  return { verifyPlacementMessage };
};

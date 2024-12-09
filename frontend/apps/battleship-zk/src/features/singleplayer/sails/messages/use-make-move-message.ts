import { usePrepareProgramTransaction } from '@gear-js/react-hooks';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { useProgram } from '@/app/utils/sails';
import { VerificationVariables } from '@/app/utils/sails/lib/lib';

export const useMakeMoveMessage = () => {
  const program = useProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'single',
    functionName: 'makeMove',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const makeMoveMessage = async (step: number | null, verificationVariables: VerificationVariables | null) => {
    const { sessionForAccount, ...params } = await prepareEzTransactionParams();
    const { transaction } = await prepareTransactionAsync({
      args: [step, verificationVariables, sessionForAccount],
      ...params,
    });
    return transaction;
  };

  return { makeMoveMessage };
};

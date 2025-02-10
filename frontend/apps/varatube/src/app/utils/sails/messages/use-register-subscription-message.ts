import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useSignAndSend } from '@/hooks/use-sign-and-send';

import { useVaratubeProgram } from '../sails';
import { Period } from '../varatube';

type Params = {
  period: Period;
  currency_id: `0x${string}`;
  with_renewal: boolean;
};

export const useRegisterSubscriptionMessage = () => {
  const program = useVaratubeProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varatube',
    functionName: 'registerSubscription',
  });
  const { signAndSend } = useSignAndSend();

  const registerSubscriptionMessage = async ({ period, currency_id, with_renewal }: Params, options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [period, currency_id, with_renewal],
      gasLimit: { increaseGas: 20 },
    });
    signAndSend(transaction, options);
  };

  return { registerSubscriptionMessage };
};

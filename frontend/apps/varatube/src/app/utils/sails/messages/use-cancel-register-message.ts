import { usePrepareProgramTransaction } from '@gear-js/react-hooks';

import { Options, useSignAndSend } from '@/hooks/use-sign-and-send';

import { useVaratubeProgram } from '../sails';

export const useCancelSubscriptionMessage = () => {
  const program = useVaratubeProgram();
  const { prepareTransactionAsync } = usePrepareProgramTransaction({
    program,
    serviceName: 'varatube',
    functionName: 'cancelSubscription',
  });

  const { signAndSend } = useSignAndSend();

  const cancelSubscriptionMessage = async (options: Options) => {
    const { transaction } = await prepareTransactionAsync({
      args: [],
      gasLimit: { increaseGas: 20 },
    });
    await signAndSend(transaction, options);
  };

  return { cancelSubscriptionMessage };
};

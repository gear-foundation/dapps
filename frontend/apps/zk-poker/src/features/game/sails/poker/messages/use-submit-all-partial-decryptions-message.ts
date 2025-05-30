import { useAlert, useSendProgramTransaction } from '@gear-js/react-hooks';
import { useMutation } from '@tanstack/react-query';
import { getErrorMessage } from '@ui/utils';
import { usePrepareEzTransactionParams } from 'gear-ez-transactions';
import { ActorId } from 'sails-js';

import { usePokerProgram } from '@/app/utils';

type Params = {
  cardsByPlayer: Array<[ActorId, [EncryptedCard, EncryptedCard]]>;
  proofs: Array<VerificationVariables>;
};

export const useSubmitAllPartialDecryptionsMessage = () => {
  const program = usePokerProgram();
  const alert = useAlert();
  const { sendTransactionAsync } = useSendProgramTransaction({
    program,
    serviceName: 'poker',
    functionName: 'submitAllPartialDecryptions',
  });
  const { prepareEzTransactionParams } = usePrepareEzTransactionParams();

  const tx = async ({ cardsByPlayer, proofs }: Params) => {
    const { sessionForAccount: _sessionForAccount, ...params } = await prepareEzTransactionParams();
    const result = await sendTransactionAsync({ args: [cardsByPlayer, proofs], ...params, gasLimit: undefined });
    return result.awaited;
  };

  const { mutateAsync, isPending } = useMutation({
    mutationFn: tx,
    onError: (error) => alert.error(getErrorMessage(error)),
  });

  return { submitAllPartialDecryptionsMessage: mutateAsync, isPending };
};

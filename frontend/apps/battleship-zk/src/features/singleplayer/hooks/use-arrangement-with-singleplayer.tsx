import { ZkProofData } from '@/features/zk/types';
import { useAccount } from '@gear-js/react-hooks';
import { useStartGameMessage } from '../sails/messages';

export const useArrangementWithSingleplayer = () => {
  const { account } = useAccount();
  const { startGameMessage } = useStartGameMessage();

  const makeStartGameTransaction = async (zkProofData: ZkProofData) => {
    if (!account?.address) {
      throw new Error('No account');
    }

    const { proofContent, publicContent } = zkProofData;

    const transaction = await startGameMessage(proofContent, {
      hash: publicContent.publicHash,
    });

    return transaction;
  };

  return { makeStartGameTransaction };
};

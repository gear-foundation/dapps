import { useAccount } from '@gear-js/react-hooks';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'react-router-dom';

import { ADDRESS } from '@/app/consts';

type Params = {
  isWaitingShuffleVerification: boolean;
};

const useZkBackend = ({ isWaitingShuffleVerification }: Params) => {
  const { gameId } = useParams();
  const { account } = useAccount();

  const getZkTask = async () => {
    if (!account || !gameId) return;

    try {
      const res = await fetch(
        `${ADDRESS.ZK_POKER_BACKEND}/api/poker/task?lobbyAddress=${gameId}&playerAddress=${account.decodedAddress}`,
      );

      const proofData = (await res.json()) as string;
      console.log('🚀 ~ getTask ~ proofData:', proofData);

      // const res = await fetch(`${ADDRESS.ZK_POKER_BACKEND}/api/poker/task`, {
      //   method: 'POST',
      //   body: JSON.stringify(payload),
      //   headers: {
      //   'Content-Type': 'application/json',
      // },

      return proofData;
    } catch (error) {
      console.error(error);
      throw new Error('Failed to fetch proof data');
    }
  };

  const { data: zkTask, isLoading: isLoadingZkTask } = useQuery({
    queryKey: ['zk-task', gameId, account?.decodedAddress, isWaitingShuffleVerification],
    queryFn: getZkTask,
    enabled: isWaitingShuffleVerification && !!account?.decodedAddress && !!gameId,
  });

  return { zkTask, isLoadingZkTask };
};

export { useZkBackend };

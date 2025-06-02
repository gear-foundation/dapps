import { useAccount } from '@gear-js/react-hooks';

import { ZkProofData } from '@/features/zk/types';

import { useVerifyPlacementMessage } from '../sails/messages';

import { useMultiplayerGame } from './use-multiplayer-game';

type GameType = 'single' | 'multi';

export const useArrangementWithMultiplayer = () => {
  const { account } = useAccount();
  const { verifyPlacementMessage } = useVerifyPlacementMessage();
  const { game } = useMultiplayerGame();
  const gameType: GameType = 'multi';

  const makeStartGameTransaction = async (zkProofData: ZkProofData) => {
    if (!account?.address) {
      throw new Error('No account');
    }

    if (!game) {
      throw new Error('No game specified');
    }

    const { proofContent, publicContent } = zkProofData;

    const transaction = await verifyPlacementMessage(
      proofContent,
      {
        hash: publicContent.publicHash,
      },
      game.admin,
    );

    return transaction;
  };

  return { gameType, makeStartGameTransaction };
};

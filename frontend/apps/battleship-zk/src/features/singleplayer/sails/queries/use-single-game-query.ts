import { useProgram } from '@/app/utils/sails';
import { useAccount } from '@gear-js/react-hooks';

export const useSingleGameQuery = () => {
  const { account } = useAccount();
  const program = useProgram();

  const gameQuery = (playerId: string) => {
    if (!account?.decodedAddress) {
      return;
    }

    return program.single.game(playerId, account.decodedAddress);
  };

  return gameQuery;
};

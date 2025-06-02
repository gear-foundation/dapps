import { useAccount } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils/sails';

export const useMultiGameQuery = () => {
  const { account } = useAccount();
  const program = useProgram();

  const gameQuery = (playerId: string) => {
    if (!account?.decodedAddress || !program) {
      return;
    }

    return program.multiple.game(playerId, account.decodedAddress);
  };

  return gameQuery;
};

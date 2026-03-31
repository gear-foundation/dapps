import { useAccount } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils/sails';

export const useSingleGameQuery = () => {
  const { account } = useAccount();
  const program = useProgram();

  const gameQuery = (playerId: string) => {
    if (!account?.decodedAddress || !program) {
      return;
    }

    return program.single.game(playerId).withAddress(account.decodedAddress).call();
  };

  return gameQuery;
};

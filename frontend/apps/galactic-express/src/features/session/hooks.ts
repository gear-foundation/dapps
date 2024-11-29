import { useAccount } from '@gear-js/react-hooks';
import { useAtomValue } from 'jotai';
import { CURRENT_GAME_ATOM } from 'atoms';
import { useGetGameQuery } from 'app/utils';

function useLaunchState() {
  const { account } = useAccount();
  const currentGame = useAtomValue(CURRENT_GAME_ATOM);

  const { game } = useGetGameQuery(currentGame || account?.decodedAddress);

  return game;
}
export { useLaunchState };

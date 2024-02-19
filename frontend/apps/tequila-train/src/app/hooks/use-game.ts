import { useApp, useGame } from 'app/context';
import { useEffect, useMemo } from 'react';
import { useAccount, useSendMessageHandler } from '@gear-js/react-hooks';
import { ENV } from 'app/consts';
import meta from 'assets/meta/tequila_train.meta.txt';

import { IGame, IState } from '../types/game';
import { useProgramMetadata, useReadState } from './use-metadata';

const programIdGame = ENV.game;

function useGameState() {
	const { account } = useAccount();

  const payloadAll = useMemo(() => ({ All: null }), []);


  const payloadGame = useMemo(
		() =>
    account?.decodedAddress
				? {
					GetGame: { player_id : account?.decodedAddress },
				}
				: undefined,

		// eslint-disable-next-line react-hooks/exhaustive-deps
		[account?.decodedAddress],
	);

	const { state: game } = useReadState<IGame | null>({
		programId: programIdGame,
		meta,
		payload: payloadGame,
	});

	const { state } = useReadState<{All: IState}>({
		programId: programIdGame,
		meta,
		payload: payloadAll,
	});

	return { state, game };
}

export const useInitGame = () => {
  const { setIsAllowed } = useApp();
  const { account } = useAccount();
  const { game, state } = useGameState()

  const { setState, setGame, setPlayers, setIsAdmin, setTimer } = useGame();

  useEffect(() => {
    if (state) {
      setState(state?.All);
    }

    if (game && game.Game && game.Game[0]) {
      const isAdmin = game.Game[0].admin === account?.decodedAddress;
      setIsAdmin(isAdmin);

      setGame(game.Game[0]);
      if (game.Game[0]) {
        const initialSeconds = game.Game[1] && parseInt(game.Game[1].replace(',', '')) / 1000;
        setTimer(Number(initialSeconds));

      
        const gameState = game.Game[0].gameState;
        
        if (game.Game[0].isStarted && gameState) {
          setPlayers(gameState.players);
          setIsAllowed(account?.decodedAddress === gameState.players[+gameState.currentPlayer].id);
        } else {
          setPlayers([]);
          setIsAllowed(false);
        }
      } else {
        setIsAdmin(false);
        setPlayers([]);
        setIsAllowed(false);
      }
    } else {
      setGame(null);
      setIsAdmin(false);
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [game, state, account]);
};


export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessageHandler(programIdGame, metadata, {
    disableAlerts: true,
    isMaxGasLimit: true,
  });
}

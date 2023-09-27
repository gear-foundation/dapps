import { useApp, useGame } from 'app/context';
import { useEffect } from 'react';
import { useAccount, useReadWasmState, useSendMessage } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { ENV } from 'app/consts';
import meta from 'assets/meta/tequila_train.meta.txt';
import metaWasm from 'assets/meta/tequila_state.meta.wasm';
import { GameWasmStateResponse, IGameState } from '../types/game';
import { useProgramMetadata, useStateMetadata, useReadState } from './use-metadata';

export function useInitGame() {
  const { setIsAllowed, setOpenWinnerPopup } = useApp();
  const { account } = useAccount();
  const { setGame, setPlayers, gameWasm } = useGame();
  const { state } = useReadState<IGameState>({ programId: ENV.game, meta });

  useEffect(() => {
    setGame(state);
    if (state && account && state.isStarted && gameWasm) {
      setPlayers(state.players);

      setIsAllowed(account.decodedAddress === state.players[+state.gameState?.currentPlayer][0]);

      if (state.gameState?.state?.winner) {
        setOpenWinnerPopup(true);
      }
    } else {
      setPlayers([]);
      setIsAllowed(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, account?.address, gameWasm]);
}

export function useGameMessage() {
  const metadata = useProgramMetadata(meta);
  return useSendMessage(ENV.game, metadata, { isMaxGasLimit: true });
}

export function useWasmState(argument?: AnyJson) {
  const { game, setGameWasm, setPlayerTiles } = useGame();
  const programMetadata = useProgramMetadata(meta);
  const stateMetadata = useStateMetadata(metaWasm);

  const programId = game?.isStarted ? ENV.game : undefined;
  const wasm = stateMetadata?.buffer;
  const functionName = 'game_state';
  const payload = '0x';

  const { state } = useReadWasmState<GameWasmStateResponse>({
    programId,
    wasm,
    functionName,
    argument,
    programMetadata,
    payload,
  });

  useEffect(() => {
    setGameWasm(state);

    if (state) {
      setPlayerTiles(state.playersTiles[+state.currentPlayer]);
    } else {
      setPlayerTiles(undefined);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state]);
}

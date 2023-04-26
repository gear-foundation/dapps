import { useApp, useGame } from 'app/context';
import { useEffect, useState } from 'react';
import { useAccount, useAlert, useApi, useReadFullState } from '@gear-js/react-hooks';
import { useMetadata, useWasmMetadata } from './use-metadata';
import { ENV } from 'app/consts';
import meta from 'assets/meta/tequila_train.meta.txt';
import metaWasm from 'assets/meta/tequila_state.meta.wasm';
import { useSendMessage } from './use-send-message';
import { GameWasmStateResponse, IGameState } from '../types/game';
import { HexString } from '@polkadot/util/types';
import { getStateMetadata, MessagesDispatched } from '@gear-js/api';
import { AnyJson } from '@polkadot/types/types';

function useReadGameState<T>() {
  const { metadata } = useMetadata(meta);
  return useReadFullState<T>(ENV.game, metadata);
}

export function useInitGame() {
  const { setIsAllowed, setOpenWinnerPopup } = useApp();
  const { account } = useAccount();
  const { setGame, setPlayers } = useGame();
  const { state } = useReadGameState<IGameState>();

  useEffect(() => {
    setGame(state);
    if (state && account && state.isStarted) {
      setPlayers(state.players.players);
      setIsAllowed(account.decodedAddress === state.players.players[state.gameState?.currentPlayer][0]);

      if (state.gameState?.state?.winner) {
        setOpenWinnerPopup(true);
      }
    } else {
      setPlayers([]);
      setIsAllowed(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, account]);
}

export function useGameMessage() {
  const { metadata } = useMetadata(meta);
  return useSendMessage(ENV.game, metadata);
}

export function useWasmState(payload?: AnyJson, isReadOnError?: boolean) {
  const { api } = useApi();
  const { game } = useGame();
  const { setGameWasm, setPlayerTiles } = useGame();
  const alert = useAlert();
  const [state, setState] = useState<GameWasmStateResponse>();
  const [error, setError] = useState('');
  const [isStateRead, setIsStateRead] = useState(true);

  const { buffer } = useWasmMetadata(metaWasm);

  const programId: HexString | undefined = ENV.game;
  const wasm: Buffer | Uint8Array | undefined = buffer;
  const functionName: string | undefined = 'game_state';
  const setupReady = !!(programId && wasm && functionName && game?.isStarted);

  const resetError = () => setError('');

  const readWasmState = () => {
    if (!setupReady) return;

    return getStateMetadata(wasm).then((stateMetadata) =>
      api.programState.readUsingWasm({ programId, wasm, fn_name: functionName, argument: payload }, stateMetadata),
    );
  };

  const readState = (isInitLoad?: boolean) => {
    if (isInitLoad) setIsStateRead(false);

    readWasmState()
      ?.then((codecState) => codecState.toJSON())
      .then((result) => {
        setState(result as unknown as GameWasmStateResponse);
        if (!isReadOnError) setIsStateRead(true);
      })
      .catch(({ message }: Error) => setError(message))
      .finally(() => {
        if (isReadOnError) setIsStateRead(true);
      });
  };

  useEffect(() => {
    if (error) alert.error(error);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [error]);

  useEffect(() => {
    readState(true);
    resetError();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [programId, wasm, functionName, game?.isStarted]);

  const handleStateChange = ({ data }: MessagesDispatched) => {
    const changedIDs = data.stateChanges.toHuman() as HexString[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) readState();
  };

  useEffect(() => {
    if (!setupReady) return;

    const unsub = api?.gearEvents.subscribeToGearEvent('MessagesDispatched', handleStateChange);

    return () => {
      unsub?.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, programId, wasm, functionName]);

  useEffect(() => {
    // console.log('wasm state: ', state);
    setGameWasm(state);

    if (state) {
      setPlayerTiles(state.playersTiles[state.currentPlayer]);
    } else {
      setPlayerTiles(undefined);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state]);
}

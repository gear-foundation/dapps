import {
  useAccount,
  useAlert,
  useApi,
  useHandleCalculateGas as useCalculateGasNative,
  useSendMessage,
} from '@gear-js/react-hooks';
import { useEffect, useMemo } from 'react';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import isEqual from 'lodash.isequal';
import { IDecodedReplyGame, IGameInstance, IQueryResponseConfig, IQueryResponseGame } from './types';
import { configAtom, countdownAtom, gameAtom, pendingAtom, stateChangeLoadingAtom } from './store';
import { ADDRESS } from './consts';
import { useOnceReadState } from '@/app/hooks/use-once-read-state';
import { useWatchMessages } from '@/app/hooks/use-watch-messages';
import { toNumber, withoutCommas } from '@/app/utils';
import { HexString, ProgramMetadata } from '@gear-js/api';
import { AnyJson, AnyNumber } from '@polkadot/types/types';
import { useAccountAvailableBalance } from '../account-available-balance/hooks';

const programIdGame = ADDRESS.GAME;

export function useGame() {
  const setGameState = useSetAtom(gameAtom);
  const gameState = useAtomValue(gameAtom);
  const setConfigState = useSetAtom(configAtom);
  const configState = useAtomValue(configAtom);
  const setCountdown = useSetAtom(countdownAtom);
  const countdown = useAtomValue(countdownAtom);

  const updateCountdown = (game: IGameInstance) => {
    setCountdown((prev) => {
      const timeLeft = toNumber(game.lastTime) + toNumber(configState?.turnDeadlineMs || '0');
      const isPassed = Date.now() - timeLeft > 0;
      const isNew = prev?.value !== game.lastTime;

      return isNew ? { value: game.lastTime, isActive: isNew && !isPassed } : prev;
    });
  };

  const updateGame = (game: IGameInstance) => {
    setGameState(game);
    updateCountdown(game);
  };

  const clearGame = () => {
    setGameState(undefined);
    setCountdown(undefined);
  };

  const resetGame = () => {
    setGameState(null);
    setCountdown(undefined);
  };

  return {
    resetGame,
    setGameState,
    gameState,
    setCountdown,
    countdown,
    setConfigState,
    configState,
    updateCountdown,
    updateGame,
    clearGame,
  };
}

export function useOnceGameState(metadata?: ProgramMetadata) {
  const { account } = useAccount();

  const payloadGame = useMemo(
    () => (account?.decodedAddress ? { Game: { player_id: account.decodedAddress } } : undefined),
    [account?.decodedAddress],
  );
  const payloadConfig = useMemo(() => ({ Config: null }), []);

  const {
    state: stateConfig,
    error: configError,
    handleReadState: triggerConfig,
  } = useOnceReadState<IQueryResponseConfig>({
    programId: programIdGame,
    payload: payloadConfig,
    meta: metadata,
  });

  const {
    state: stateGame,
    error: gameError,
    handleReadState: triggerGame,
  } = useOnceReadState<IQueryResponseGame>({
    programId: programIdGame,
    payload: payloadGame,
    meta: metadata,
  });

  return {
    stateGame,
    stateConfig,
    error: gameError || configError,
    triggerGame,
    triggerConfig,
  };
}

export const useInitGame = () => {
  const { account } = useAccount();
  const { gameState } = useGame();

  return {
    isGameReady: account?.decodedAddress ? gameState !== undefined : true,
  };
};
export const useInitGameSync = (metadata?: ProgramMetadata) => {
  const { isApiReady, api } = useApi();
  const { account } = useAccount();
  const { stateGame, stateConfig, error, triggerGame, triggerConfig } = useOnceGameState(metadata);
  const { updateGame, resetGame, setConfigState } = useGame();

  useEffect(() => {
    if (!isApiReady || !api || !metadata || stateConfig?.Config) return;

    triggerConfig(metadata);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, api, metadata, account?.decodedAddress]);

  useEffect(() => {
    if (!isApiReady || !api || !metadata || !stateConfig?.Config) return;

    triggerGame(metadata);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, api, metadata, stateConfig?.Config, account?.decodedAddress]);

  useEffect(() => {
    if (!stateConfig?.Config) return;

    setConfigState(stateConfig.Config);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stateConfig?.Config]);

  useEffect(() => {
    if (stateGame === undefined) return;

    const game = stateGame?.Game;

    game ? updateGame(game) : resetGame();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stateGame]);

  return {
    errorGame: error,
  };
};

export function useGameMessage(meta: ProgramMetadata) {
  return useSendMessage(programIdGame, meta, { disableAlerts: true });
}

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);
  return { pending, setPending };
}

export function useSubscriptionOnGameMessage(meta: ProgramMetadata) {
  const { gameState, updateGame } = useGame();
  const { subscribe, unsubscribe, reply, isOpened } = useWatchMessages<IDecodedReplyGame>(meta);
  const setIsLoading = useSetAtom(stateChangeLoadingAtom);

  useEffect(() => {
    if (!isOpened) return;
    const game = reply?.MoveMade?.game || reply?.GameStarted?.game || reply?.GameFinished?.game;

    if (game && !isEqual(game.board, gameState?.board)) {
      updateGame(game);
      unsubscribe();
      setIsLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [reply, isOpened]);

  return {
    subscribe,
    unsubscribe,
    reply,
    isOpened,
  };
}

export const useHandleCalculateGas = (address: HexString, meta: ProgramMetadata) => {
  const { availableBalance } = useAccountAvailableBalance();
  const calculateGasNative = useCalculateGasNative(address, meta);

  const alert = useAlert();

  return (initPayload: AnyJson, value?: AnyNumber | undefined) => {
    const balance = Number(withoutCommas(availableBalance?.value || ''));
    const existentialDeposit = Number(withoutCommas(availableBalance?.existentialDeposit || ''));

    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`);
    }

    return calculateGasNative(initPayload, value);
  };
};

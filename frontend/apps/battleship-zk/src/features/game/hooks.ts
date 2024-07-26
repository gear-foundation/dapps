import { useAtom } from 'jotai';
import { useLocation } from 'react-router-dom';
import { TransactionBuilder } from 'sails-js';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { VerificationVariables } from '@/app/utils/sails/lib/lib';
import { ROUTES } from '@/app/consts';
import { gameModeAtom, pendingAtom } from './store';
import { useProofShipHit } from '../zk/hooks/use-proof-ship-hit';
import { GameType } from './types';
import { getVerificationVariables } from '../zk/utils';

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useIsLocationGamePage() {
  const { pathname } = useLocation();

  return pathname === ROUTES.GAME;
}

export function useGameMode() {
  const [gameMode, setGameMode] = useAtom(gameModeAtom);

  const resetGameMode = () => {
    setGameMode(null);
  };

  return { gameMode, setGameMode, resetGameMode };
}

export function useMoveTransaction(
  gameType: GameType,
  makeMoveMessage: (
    step: number | null,
    verificationVariables: VerificationVariables | null,
    gameId?: string,
  ) => Promise<TransactionBuilder<null>>,
  triggerGame: () => Promise<void>,
) {
  const { getProofData, clearProofData } = useProofShipHit();
  const { account } = useAccount();
  const { api } = useApi();

  const moveTransaction = async (step: number | null, gameId?: string) => {
    if (!account?.address || !api) {
      return;
    }
    const proofDataHit = getProofData(gameType);
    const verificationVariables = getVerificationVariables(proofDataHit);
    const hitTransaction = await makeMoveMessage(step, verificationVariables, gameId);

    const { response } = await hitTransaction.signAndSend();
    await response();
    clearProofData(gameType);

    return triggerGame();
  };

  return moveTransaction;
}

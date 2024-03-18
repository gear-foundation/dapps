import { useMemo } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { ENV } from '@/app/consts';
import { useReadState } from './use-metadata';
import meta from '@/assets/meta/vara_man.meta.txt';
import { GameState, IGameConfig } from '@/app/types/game';

export const programIdGame = ENV.GAME;

export function useGameState() {
  const { account } = useAccount();

  const payloadTournamentGame = useMemo(
    () =>
      account?.decodedAddress
        ? {
          GetTournament: account.decodedAddress,
        }
        : undefined,
    [account?.decodedAddress],
  );


  const payloadConfig = useMemo(() => ({ Config: null }), []);
  const payloadAdmins = useMemo(() => ({ Admins: null }), []);


  const { state: game } = useReadState<GameState>({
    programId: programIdGame,
    meta,
    payload: payloadTournamentGame,
  });

  const { state: config } = useReadState<{ Config: IGameConfig | null }>({
    programId: programIdGame,
    meta,
    payload: payloadConfig,
  });


  const { state: admins } = useReadState<{ Admins: string[] }>({
    programId: programIdGame,
    meta,
    payload: payloadAdmins,
  });


  return { game, config, admins };
}

import { useMemo } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { ENV } from '@/app/consts';
import { useReadState } from './use-metadata';
import meta from '@/assets/meta/vara_man.meta.txt';
import { IGameConfig, ITournamentGameInstance } from '@/app/types/game';

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


  const payloadAllState = useMemo(() => ({ All: null }), []);
  const payloadConfig = useMemo(() => ({ Config: null }), []);
  const payloadAdmins = useMemo(() => ({ Admins: null }), []);

  const { state: allState } = useReadState<any>({
    programId: programIdGame,
    meta,
    payload: payloadAllState,
  });


  const { state: tournament } = useReadState<{ Tournament: ITournamentGameInstance }>({
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

  return { allState, tournament, config, admins };
}

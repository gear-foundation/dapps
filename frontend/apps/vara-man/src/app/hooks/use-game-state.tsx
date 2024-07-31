import { useMemo } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { useReadState } from './use-metadata';
import meta from '@/assets/meta/vara_man.meta.txt';
import { IGameConfig, ITournamentGameInstance } from '@/app/types/game';
import { useDnsProgramIds } from '@dapps-frontend/hooks';

export function useGameState() {
  const { programId } = useDnsProgramIds();

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
    programId,
    meta,
    payload: payloadAllState,
  });

  const { state: tournament } = useReadState<{ Tournament: ITournamentGameInstance }>({
    programId,
    meta,
    payload: payloadTournamentGame,
  });

  const { state: config } = useReadState<{ Config: IGameConfig | null }>({
    programId,
    meta,
    payload: payloadConfig,
  });

  const { state: admins } = useReadState<{ Admins: string[] }>({
    programId,
    meta,
    payload: payloadAdmins,
  });

  return { allState, tournament, config, admins };
}

import { useAtom } from 'jotai';
import { useProgramEvent, useAccount } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { isBattleCanceledAtom } from '@/features/game/store';

export function useEventBattleCanceledSubscription(currentGameId?: string) {
  const program = useProgram();
  const [isBattleCanceled, setIsBattleCanceled] = useAtom(isBattleCanceledAtom);
  const { account } = useAccount();

  const onBattleCanceled = ({ game_id }: { game_id: string }) => {
    if (currentGameId === game_id && account?.decodedAddress !== currentGameId) {
      setIsBattleCanceled(true);
    }
  };

  const onRegisterCanceled = ({ player_id }: { player_id: string }) => {
    if (account?.decodedAddress === player_id) {
      setIsBattleCanceled(true);
    }
  };

  useProgramEvent({
    program,
    serviceName: 'battle',
    functionName: 'subscribeToBattleCanceledEvent',
    onData: onBattleCanceled,
  });

  useProgramEvent({
    program,
    serviceName: 'battle',
    functionName: 'subscribeToRegisterCanceledEvent',
    onData: onRegisterCanceled,
  });

  return { isBattleCanceled, setIsBattleCanceled };
}

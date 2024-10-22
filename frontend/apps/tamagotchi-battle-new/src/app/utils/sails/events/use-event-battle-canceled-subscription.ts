import { useAtom } from 'jotai';
import { useProgramEvent, useAccount } from '@gear-js/react-hooks';
import { useProgram } from '@/app/utils';
import { isBattleCanceledAtom } from '@/features/game/store';

export function useEventBattleCanceledSubscription(currentGameId?: string) {
  const program = useProgram();
  const [isBattleCanceled, setIsBattleCanceled] = useAtom(isBattleCanceledAtom);
  const { account } = useAccount();

  const onData = ({ game_id }: { game_id: string }) => {
    if (currentGameId === game_id && account?.decodedAddress !== currentGameId) {
      setIsBattleCanceled(true);
    }
  };

  useProgramEvent({
    program,
    serviceName: 'battle',
    functionName: 'subscribeToBattleCanceledEvent',
    onData,
  });

  return { isBattleCanceled, setIsBattleCanceled };
}

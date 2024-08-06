import { useProgram } from '@/app/utils/sails';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';
import { SERVICE_NAME } from '../../consts';
import { useAtomValue } from 'jotai';
import { gameEndResultAtom } from '../../atoms';
import { useEffect } from 'react';
import { usePending } from '@/features/game/hooks';

export const useRemainingTimeQuery = () => {
  const { account } = useAccount();
  const gameEndResult = useAtomValue(gameEndResultAtom);
  const program = useProgram();
  const { pending } = usePending();

  const {
    data: remainingTime,
    refetch,
    isFetching,
  } = useProgramQuery({
    program,
    serviceName: SERVICE_NAME,
    functionName: 'getRemainingTime',
    args: [account?.decodedAddress || ''],
    query: { enabled: false },
  });

  useEffect(() => {
    if (!gameEndResult && !pending) {
      refetch();
    }
  }, [pending, gameEndResult]);

  return isFetching ? undefined : remainingTime;
};

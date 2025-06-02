import { useAccount, useProgramQuery } from '@gear-js/react-hooks';
import { useAtomValue } from 'jotai';
import { useEffect } from 'react';

import { useProgram } from '@/app/utils/sails';
import { usePending } from '@/features/game/hooks';

import { gameEndResultAtom } from '../../atoms';
import { SERVICE_NAME } from '../../consts';

export const useRemainingTimeQuery = () => {
  const { account } = useAccount();
  const gameEndResult = useAtomValue(gameEndResultAtom);
  const program = useProgram();
  const { pending } = usePending();

  const { data, refetch, isFetching } = useProgramQuery({
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

  const remainingTime = gameEndResult?.winner === 'Bot' ? 0 : data;

  return isFetching ? undefined : remainingTime;
};

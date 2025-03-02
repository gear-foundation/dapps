import { HexString } from '@gear-js/api';
import { useAccount, useProgramQuery } from '@gear-js/react-hooks';
import { useAtomValue } from 'jotai';

import { useProgram } from '@/app/utils';
import { CURRENT_GAME_ADMIN_ATOM } from '@/atoms';

export const useGetGameSessionQuery = (address?: HexString | null, disabled?: boolean) => {
  const program = useProgram();
  const { account } = useAccount();
  const admin = useAtomValue(CURRENT_GAME_ADMIN_ATOM);

  const gameAddress = address || admin || account?.decodedAddress;

  const { data, refetch, isFetching, isFetched, error } = useProgramQuery({
    program,
    serviceName: 'syndote',
    functionName: 'getGameSession',
    args: [gameAddress || '0x'],
    query: { enabled: !disabled && gameAddress ? undefined : false },
    watch: true,
  });

  return { state: data, isFetching, isFetched, refetch, error };
};

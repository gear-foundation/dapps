import { ADDRESS } from '@/app/consts';
import { useProgramMetadata } from '@/app/hooks';
import { HexString } from '@gear-js/api';
import { useSendMessageHandler } from '@gear-js/react-hooks';

import metaTxt from '@/features/game/assets/meta/battleship.meta.txt';

function useCreateSession() {
  const metadata = useProgramMetadata(metaTxt);
  const sendMessage = useSendMessageHandler(ADDRESS.GAME, metadata);

  const createSession = (accountAddress: HexString, duration: number, allowedActions: string) => {
    const key = accountAddress;
    const payload = { CreateSession: { duration, allowedActions, key } };

    sendMessage({ payload });
  };

  return createSession;
}

export { useCreateSession };

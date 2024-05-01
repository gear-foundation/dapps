import { useAccount } from '@gear-js/react-hooks';
import { useReadGameSessionState, useSyndoteMessage } from './metadata';

export const useQuitGame = () => {
  const { state, isStateRead } = useReadGameSessionState();
  const { isMeta, sendMessage } = useSyndoteMessage();
  const { account } = useAccount();
  const { adminId } = state || {};

  const cancelGame = () => {
    if (!isMeta || !account?.decodedAddress || !isStateRead) {
      return;
    }

    const payload = {
      CancelGameSession: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

  const exitGame = () => {
    if (!isMeta || !account?.decodedAddress || !isStateRead) {
      return;
    }

    const payload = {
      ExitGame: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

  const deleteGame = () => {
    const payload = {
      DeleteGame: {
        adminId,
      },
    };

    sendMessage({
      payload,
    });
  };

  return { cancelGame, exitGame, deleteGame };
};

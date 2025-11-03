import { useAccount } from '@gear-js/react-hooks';

import {
  useCancelGameSessionMessage,
  useDeleteGameMessage,
  useExitGameMessage,
  useGetGameSessionQuery,
} from '@/app/utils';

export const useQuitGame = () => {
  const { state, isFetched } = useGetGameSessionQuery();
  const { account } = useAccount();
  const { admin_id: adminId } = state || {};

  const { cancelGameSessionMessage } = useCancelGameSessionMessage();
  const { exitGameMessage } = useExitGameMessage();
  const { deleteGameMessage } = useDeleteGameMessage();
  const cancelGame = () => {
    if (!account?.decodedAddress || !isFetched || !adminId) {
      return;
    }

    void cancelGameSessionMessage({ adminId });
  };

  const exitGame = () => {
    if (!account?.decodedAddress || !isFetched || !adminId) {
      return;
    }

    void exitGameMessage({ adminId });
  };

  const deleteGame = () => {
    if (!account?.decodedAddress || !isFetched || !adminId) {
      return;
    }

    void deleteGameMessage({ adminId });
  };

  return { cancelGame, exitGame, deleteGame };
};

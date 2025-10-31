import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';

import { Loader } from '@/components';
import SelectGameMode from '@/features/game/components/select-game-mode/select-game-mode';
import { useGameMode } from '@/features/game/hooks';
import { CreateGameForm } from '@/features/multiplayer/components/create-game-form';
import { JoinGameForm } from '@/features/multiplayer/components/join-game-form';
import { Registration } from '@/features/multiplayer/components/registration';
import { useInitMultiplayerGame, useMultiplayerGame } from '@/features/multiplayer/hooks';
import { clearZkData } from '@/features/zk/utils';

import Login from './login';

export default function Home() {
  const { account } = useAccount();
  const { gameMode, resetGameMode } = useGameMode();
  const { isLoading, isActiveGame } = useInitMultiplayerGame();
  const { game } = useMultiplayerGame();

  const handleCancel = () => {
    resetGameMode();
  };

  useEffect(() => {
    if (game === null && account?.address) {
      clearZkData('multi', account);
    }
  }, [account, game]);

  return (
    <>
      {!account?.decodedAddress && <Login />}
      {account?.decodedAddress && isLoading && <Loader />}
      {account?.decodedAddress && !isLoading && !isActiveGame && (
        <>
          {!gameMode && <SelectGameMode />}
          {gameMode === 'single' && <Login />}
          {gameMode === 'find' && <JoinGameForm onCancel={handleCancel} />}
          {gameMode === 'create' && <CreateGameForm onCancel={handleCancel} />}
        </>
      )}
      {account?.decodedAddress && !isLoading && isActiveGame && <Registration />}
    </>
  );
}

import { useAccount } from '@gear-js/react-hooks';
import Login from './login';
import SelectGameMode from '@/features/game/components/select-game-mode/select-game-mode';
import { useGameMode } from '@/features/game/hooks';
import { JoinGameForm } from '@/features/multiplayer/components/join-game-form';
import { CreateGameForm } from '@/features/multiplayer/components/create-game-form';
import { useInitMultiplayerGame, useMultiplayerGame } from '@/features/multiplayer/hooks';
import { Loader } from 'lucide-react';
import { Registration } from '@/features/multiplayer/components/registration';
import { useEffect } from 'react';
import { clearZkData } from '@/features/zk/utils';

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
  }, [isActiveGame, account]);

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

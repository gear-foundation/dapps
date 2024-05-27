import { useAccount } from '@gear-js/react-hooks';
import Login from './login';
import SelectGameMode from '@/features/game/components/select-game-mode/select-game-mode';
import { useGameMode } from '@/features/game/hooks';
import { JoinGameForm } from '@/features/game/components/join-game-form';
import { CreateGameForm } from '@/features/game/components/create-game-form';

export default function Home() {
  const { account } = useAccount();
  const { gameMode, resetGameMode } = useGameMode();

  const handleCancel = () => {
    resetGameMode();
  };

  return (
    <>
      {!account?.decodedAddress && <Login />}
      {account && (
        <>
          {!gameMode && <SelectGameMode />}
          {gameMode === 'single' && <Login />}
          {gameMode === 'find' && <JoinGameForm onCancel={handleCancel} />}
          {gameMode === 'create' && <CreateGameForm onCancel={handleCancel} />}
        </>
      )}
    </>
  );
}

import { Background } from '@/features/game/components/background';
import { CreateGameForm } from '@/features/game/components/sections/create-game-form';

export function CreateGame() {
  return (
    <>
      <Background>
        <CreateGameForm />
      </Background>
    </>
  );
}

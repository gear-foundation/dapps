import { CreateGameForm } from '@/features/game/components/sections/create-game-form';

import { Background } from '../features/game/components/background';

export default function CreateGame() {
  return (
    <>
      <Background>
        <CreateGameForm />
      </Background>
    </>
  );
}

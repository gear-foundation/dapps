import { GameNav } from '@/components/sections/game/game-nav';
import { GameField } from '@/components/sections/game/game-field';
import GameCore from './game-canvas';
import { GameProviderScore } from '@/app/context/ctx-game-score';

type GameInitProps = {};

export function GameInit({}: GameInitProps) {
  return (
    <div>
      <GameProviderScore>
        <GameNav />
        <GameCore />
      </GameProviderScore>
    </div>
  );
}

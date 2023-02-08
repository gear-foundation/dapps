import clsx from 'clsx';
import { BattleRoundStatsAvatar } from 'components/sections/battle-round-stats-avatar';
import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { BattleStateResponse } from 'app/types/battles';

const BattleTurnArrows = ({ isReverse }: { isReverse: boolean }) => (
  <div className={clsx('flex', isReverse && 'rotate-180')}>
    <Icon
      name="battle-next-step"
      className="w-6 xl:w-10 aspect-[1/2] text-white animate-battle-turn-1 transition-opacity"
    />
    <Icon
      name="battle-next-step"
      className="w-6 xl:w-10 aspect-[1/2] text-white animate-battle-turn-2 transition-opacity"
    />
    <Icon
      name="battle-next-step"
      className="w-6 xl:w-10 aspect-[1/2] text-white animate-battle-turn-3 transition-opacity"
    />
  </div>
);

export const BattleRoundStats = ({ battle }: { battle: BattleStateResponse }) => {
  const { players, currentPlayer } = useBattle();
  return (
    <div className="flex gap-10 justify-between items-center">
      <BattleRoundStatsAvatar
        state={battle?.state}
        isWinner={Boolean(battle.players[battle.currentWinner])}
        tamagotchi={players[0]}
      />
      {battle?.state === 'GameIsOn' && <BattleTurnArrows isReverse={players[1].tmgId === currentPlayer} />}
      <BattleRoundStatsAvatar
        state={battle?.state}
        isWinner={Boolean(battle.players[battle.currentWinner])}
        tamagotchi={players[1]}
        isReverse
      />
    </div>
  );
};

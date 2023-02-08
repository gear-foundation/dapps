import { TamagotchiBattleTopStats } from '../../tamagotchi/tamagotchi-battle-top-stats';
import clsx from 'clsx';
import { Icon } from '../../ui/icon';
import { useBattle } from '../../../app/context';
import { BattleStateResponse } from '../../../app/types/battles';

export const BattleRoundStats = ({ battle }: { battle: BattleStateResponse }) => {
  const { players: warriors } = useBattle();
  return (
    <div className="flex gap-10 justify-between items-center">
      <TamagotchiBattleTopStats
        state={battle?.state}
        isWinner={Boolean(battle.players[battle.currentWinner])}
        tamagotchi={warriors[0]}
      />
      {battle?.state === 'GameIsOn' && (
        <div className={clsx('flex', battle?.currentTurn === 1 && 'rotate-180')}>
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
      )}
      <TamagotchiBattleTopStats
        state={battle?.state}
        isWinner={Boolean(battle.players[battle.currentWinner])}
        tamagotchi={warriors[1]}
        isReverse
      />
    </div>
  );
};

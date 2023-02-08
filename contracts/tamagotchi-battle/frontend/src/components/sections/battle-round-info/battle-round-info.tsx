import { TamagotchiBattleInfoCard } from '../../tamagotchi/tamagotchi-battle-info-card';
import { useBattle } from 'app/context';

export const BattleRoundInfo = () => {
  const { players } = useBattle();
  return (
    <div className="flex gap-10 justify-between mt-8 xl:mt-10">
      <div className="basis-[445px] flex flex-col">
        <TamagotchiBattleInfoCard tamagotchi={players[0]} />
      </div>
      <div className="basis-[445px] flex flex-col">
        <TamagotchiBattleInfoCard tamagotchi={players[1]} />
      </div>
    </div>
  );
};

import { TamagotchiBattleInfoCard } from 'components/cards/tamagotchi-battle-info-card';
import { useBattle } from 'app/context';
import { Icon } from 'components/ui/icon';

export const BattleRoundInfo = () => {
  const { rivals, currentPlayer, players } = useBattle();
  return (
    <div className="relative flex gap-10 justify-between mt-4 xxl:mt-7">
      <div className="basis-[40%] flex justify-center">
        <TamagotchiBattleInfoCard tamagotchi={rivals[0]} isActive={rivals[0].tmgId === currentPlayer} />
      </div>
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
        <div className="border border-white/10 bg-white/[3%] backdrop-blur-md p-6 pt-5 rounded-2xl font-kanit text-base text-white/60 tracking-wider">
          <h3 className="font-normal text-center">
            Participants: <b className="inline-block ml-1 text-xl font-semibold text-white">{players.length}</b>
          </h3>
          <div className="flex items-center gap-12 mt-4">
            <div className="flex items-center gap-2">
              <Icon name="participants-alive" className="w-6 h-6 shrink-0" />
              <p className="flex items-center">
                Alive:{' '}
                <b className="inline-block ml-1 text-xl font-semibold text-white">
                  {players.filter((el) => el.health).length}
                </b>
              </p>
            </div>
            <div className="flex items-center gap-2">
              <Icon name="participants-dead" className="w-6 h-6 shrink-0" />
              <p className="flex items-center">
                Dead:{' '}
                <b className="inline-block ml-1 text-xl font-semibold text-white">
                  {players.filter((el) => !el.health).length}
                </b>
              </p>
            </div>
          </div>
        </div>
      </div>
      <div className="basis-[40%] flex justify-center">
        <TamagotchiBattleInfoCard tamagotchi={rivals[1]} isActive={rivals[1].tmgId === currentPlayer} />
      </div>
    </div>
  );
};

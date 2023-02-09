import { BattleStateResponse } from 'app/types/battles';
import { useBattle } from '../../../app/context';
import { TamagotchiBattleInfoCard } from '../../tamagotchi/tamagotchi-battle-info-card';
import { TamagotchiAvatar } from '../../tamagotchi/tamagotchi-avatar';
import { getTamagotchiAgeDiff } from '../../../app/utils/get-tamagotchi-age';
import Fireworks, { FireworksHandlers } from '@fireworks-js/react';
import { useRef } from 'react';

export const BattleWinner = ({ battle }: { battle: BattleStateResponse }) => {
  const { players, currentPlayer } = useBattle();

  return (
    <section className="container flex flex-col grow">
      <BattleWinnerFireworks />
      <div className="flex gap-10 justify-center items-center mt-2 xl:mt-15">
        <div className="text-center font-bold">
          <p className="text-2xl leading-normal xl:typo-h2 truncate max-w-[19ch]">
            <span className="text-primary">{currentPlayer && battle.players[currentPlayer].name}</span>
            <br />
            Winner
          </p>
        </div>
      </div>
      <div className="relative grow flex justify-center gap-10 mt-2 xl:mt-15">
        <div className="relative w-full max-w-[450px] flex flex-col">
          <TamagotchiAvatar
            color={players[0].color}
            age={players[0].dateOfBirth}
            className="grow w-full h-full"
            isWinner
            isDead={!players[0].health}
          />
        </div>
      </div>
      <div className="relative flex gap-10 justify-center mt-4 xl:mt-7">
        <TamagotchiBattleInfoCard tamagotchi={players[0]} isActive={players[0].tmgId === currentPlayer} />
      </div>
    </section>
  );
};

const BattleWinnerFireworks = () => {
  const ref = useRef<FireworksHandlers>(null);
  return (
    <Fireworks
      ref={ref}
      options={{
        opacity: 0.5,
        intensity: 15,
        lineWidth: {
          trace: {
            min: 0,
            max: 0.15,
          },
        },
        traceLength: 1,
        hue: {
          min: 130,
          max: 160,
        },
      }}
      style={{
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        position: 'fixed',
        background: '',
        zIndex: '-1',
      }}
    />
  );
};

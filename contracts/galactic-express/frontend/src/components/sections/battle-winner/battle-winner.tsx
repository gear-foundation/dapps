import { BattleStateResponse } from 'app/types/battles';
import { TamagotchiBattleInfoCard } from 'components/cards/tamagotchi-battle-info-card';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';
import Fireworks, { FireworksHandlers } from '@fireworks-js/react';
import { useRef } from 'react';

export const BattleWinner = ({ battle }: { battle: BattleStateResponse }) => {
  const winner = battle.players[battle.currentWinner];

  return (
    <section className="container flex flex-col grow">
      <BattleWinnerFireworks />
      <div className="flex gap-10 justify-center items-center mt-2 xxl:mt-15">
        <div className="text-center font-bold">
          <p className="text-2xl leading-normal xxl:typo-h2 truncate max-w-[19ch]">
            <span className="text-primary">{winner.name}</span>
            <br />
            Winner
          </p>
        </div>
      </div>
      <div className="relative grow flex justify-center gap-10 mt-2 xxl:mt-15">
        <div className="relative w-full max-w-[450px] flex flex-col">
          <TamagotchiAvatar
            color={winner.color}
            age={winner.dateOfBirth}
            className="grow w-full h-full"
            isWinner
            isDead={!winner.health}
          />
        </div>
      </div>
      <div className="relative flex gap-10 justify-center mt-4 xxl:mt-7">
        <TamagotchiBattleInfoCard tamagotchi={winner} isActive />
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

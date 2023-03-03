import { BattleStateResponse } from 'app/types/battles';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';
import Fireworks, { FireworksHandlers } from '@fireworks-js/react';
import { useRef } from 'react';
import { TamagotchiQueueCard } from '../../cards/tamagotchi-queue-card';

export const BattleWinner = ({ battle }: { battle: BattleStateResponse }) => {
  const winner = battle.players[battle.currentWinner];

  return (
    <>
      {winner && (
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
          <div className="relative grow grid gap-10 mt-2 xxl:mt-15">
            <div className="grow grid max-h-full my-auto h-full">
              <TamagotchiAvatar
                color={winner.color}
                age={winner.dateOfBirth}
                className="max-w-full"
                // className="grow h-full mx-auto w-fit max-w-full"
                isWinner
                isDead={!winner.health}
              />
            </div>
          </div>
          <div className="relative z-1 flex gap-10 justify-center mt-4 xxl:mt-7">
            <TamagotchiQueueCard tamagotchi={winner} isActive asPlayer />
          </div>
        </section>
      )}
    </>
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

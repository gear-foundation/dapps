import { useContext, useEffect, useState } from 'react';
import { GameContext } from '@/app/context/ctx-game-score';
import { useAccount } from '@gear-js/react-hooks';

import { gameNavData } from '@/components/sections/game/game-nav.data';
import { GameNavBackground } from '@/components/sections/game/game-nav-background';
import { GameNavItem } from '@/components/sections/game/game-nav-item';
import GameNavChampions from '@/components/sections/game/game-nav-champions';

import StatsHeroImage from '@/assets/images/game/stats-hero.svg';

const data = gameNavData;

type GameNavProps = BaseComponentProps & {};

export function GameNav({}: GameNavProps) {
  const { account } = useAccount();
  const { silverCoins, goldCoins, lives, gameTime } = useContext(GameContext);
  const [formattedTimer, setFormattedTimer] = useState('00:00');
  const [timer, setTimer] = useState(gameTime);

  useEffect(() => {
    const formatTimer = (seconds: number) => {
      const minutes = Math.floor(seconds / 60);
      const remainingSeconds = seconds % 60;
      const formattedMinutes = String(minutes).padStart(2, '0');
      const formattedSeconds = String(remainingSeconds).padStart(2, '0');
      return `${formattedMinutes}:${formattedSeconds}`;
    };

    setFormattedTimer(formatTimer(timer));
  }, [timer]);

  useEffect(() => {
    const interval = setInterval(() => {
      setTimer((prevTimer) => Math.max(prevTimer - 1, 0));
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  const shortenString = (str: string, length: number) => {
    if (str.length <= length) {
      return str;
    }
    const start = str.slice(0, length / 2);
    const end = str.slice(-length / 2);
    return start + '...' + end;
  };

  const shortenedStr = account && shortenString(account.decodedAddress, 10);

  return (
    <div className="relative font-kanit">
      <GameNavBackground />

      <div className="flex items-center justify-between py-10 px-7.5">
        <div className="flex space-x-10">
          <GameNavChampions />

          <div className="flex space-x-8">
            <GameNavItem icon={data[0].icon} className={data[0].color}>
              {lives}
            </GameNavItem>
            <GameNavItem icon={data[1].icon} className={data[1].color}>
              {formattedTimer}
            </GameNavItem>
          </div>
        </div>

        <div className="absolute bottom-2 left-1/2 -translate-x-1/2 grid gap-4 text-center">
          <img width={92} height={92} src={StatsHeroImage} alt="Avatar" />

          <span className="text-test">Vara - Man</span>
        </div>

        <div className="flex space-x-10">
          <div className="flex space-x-8">
            <GameNavItem icon={data[2].icon} className={data[2].color}>
              {goldCoins}
            </GameNavItem>
            <GameNavItem icon={data[3].icon} className={data[3].color}>
              {silverCoins}
            </GameNavItem>
          </div>

          <div className="btn bg-white/[1%] shadow-white shadow-[inset_0_0_4px] px-6 flex-col items-start pt-1 pb-1.5 cursor-auto">
            <small className="text-white/60 opacity-80 font-normal text-[10px] leading-[14px]">Substrate address</small>
            <span className="leading-4">{shortenedStr}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

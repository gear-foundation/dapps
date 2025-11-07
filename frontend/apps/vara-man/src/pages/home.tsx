import { useEffect } from 'react';

import { useGame } from '@/app/context/ctx-game';
import { HomeRegister } from '@/components/sections/home/home-register';

import Game from './game';

export default function Home() {
  const { tournamentGame, previousGame, setPreviousGame } = useGame();

  useEffect(() => {
    if (tournamentGame) {
      setPreviousGame(tournamentGame);
    }
  }, [setPreviousGame, tournamentGame]);

  return <>{tournamentGame || previousGame ? <Game /> : <HomeRegister />}</>;
}

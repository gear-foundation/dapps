import { HomeRegister } from '@/components/sections/home/home-register';
import { HomeNotActive } from '@/components/sections/home/home-not-active';
import { useGame } from '@/app/context/ctx-game';
import Game from './game';
import { useInitGame } from '@/app/hooks/use-game';


export default function Home() {
  const { singleGame, tournamentGame } = useGame()

  return <>{singleGame || tournamentGame ? <Game /> : <HomeRegister />}</>;
}

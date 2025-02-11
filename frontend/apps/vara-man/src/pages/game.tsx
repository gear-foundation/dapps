import { useGame } from '@/app/context/ctx-game';
import { GameLayout as SingleGame } from '@/feature/single-game';
import { Game as TournamentGame } from '@/feature/tournament-game';

export default function GamePage() {
  const { tournamentGame, previousGame } = useGame();

  return <>{tournamentGame || previousGame ? <TournamentGame /> : <SingleGame />}</>;
}

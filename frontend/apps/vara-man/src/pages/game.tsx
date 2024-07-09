import { Game as TournamentGame } from '@/feature/tournament-game';
import { GameLayout as SingleGame } from '@/feature/single-game';
import { useGame } from '@/app/context/ctx-game';

export default function GamePage() {
  const { tournamentGame, previousGame } = useGame();

  return <>{tournamentGame || previousGame ? <TournamentGame /> : <SingleGame />}</>;
}

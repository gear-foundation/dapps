import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useGame } from '@/app/context/ctx-game';
import { GameInit } from '@/components/sections/game/game-init'

export default function Home() {
  const navigate = useNavigate();
  const { player, game } = useGame()

  useEffect(() => {
    if (game && player) {
      const findGamePlayer = game.games.find(x => x[0] === player[0])

      if (!findGamePlayer) {
        navigate('/levels');
      }
    }
  }, [navigate]);

  return (
    <>
      <GameInit />
    </>
  )
}

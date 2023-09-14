import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useGame } from '@/app/context/ctx-game';
import { GameInit } from '@/components/sections/game/game-init'

export default function Home() {
  const navigate = useNavigate();
  const { player, game, gamePlayer } = useGame()

  useEffect(() => {
    if (game && player) {

      if (!gamePlayer) {
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

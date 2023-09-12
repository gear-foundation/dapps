import { HomeRegister } from '@/components/sections/home/home-register'
import { useGame } from '@/app/context/ctx-game'
import { useNavigate } from 'react-router-dom'
import { useEffect } from 'react'

export default function Home() {
  const { player } = useGame()
  const navigate = useNavigate();

  useEffect(() => {
    if (player?.length) {
      navigate('/levels');
    }
  }, [navigate, player])


  return (
    <>
      <HomeRegister />
    </>
  )
}

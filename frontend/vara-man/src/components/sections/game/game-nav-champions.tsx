import React, { useState, memo } from 'react';
import { cn } from '@/app/utils'
import { buttonStyles } from '@gear-js/ui'
import { ChampionsPopup } from '@/components/popups/champions-popup'
import { useGame } from '@/app/context/ctx-game'

type GameNavProps = BaseComponentProps & {}

function GameNavChampions({ }: GameNavProps) {
  const [open, setOpen] = useState(false)
  const { game } = useGame()

  const sortedPlayers = game
    ? game.players.slice().sort((playerA, playerB) => {
      const [_, playerInfoA] = playerA;
      const [__, playerInfoB] = playerB;

      const totalCoinsA = playerInfoA.claimedGoldCoins + playerInfoA.claimedSilverCoins;
      const totalCoinsB = playerInfoB.claimedGoldCoins + playerInfoB.claimedSilverCoins;

      return totalCoinsB - totalCoinsA;
    })
    : [];

  return (
    <>
      <button
        className={cn('btn px-6', buttonStyles.lightGreen)}
        onClick={() => setOpen((_) => !_)}
      >
        Show champions
      </button>

      {game && <ChampionsPopup setIsOpen={setOpen} isOpen={open} players={sortedPlayers} />}
    </>
  )
}

export default memo(GameNavChampions);
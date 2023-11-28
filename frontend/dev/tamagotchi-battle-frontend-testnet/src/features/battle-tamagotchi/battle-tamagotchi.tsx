import { useBattle } from '@/features/battle-tamagotchi/context'
import { BattlePlayersQueue } from './components/battle-players-queue'
import { BattleRound } from './components/battle-round'
import { BattleWaitAdmin } from './components/battle-wait-admin'
import { BattleWaitRegistration } from './components/battle-wait-registration'
import { BattleWinner } from './components/battle-winner'

export function BattleTamagotchi() {
  const { battle, rivals, currentPairIdx, isAdmin } = useBattle()

  const gameIsOn = Boolean(
    battle &&
      ['GameIsOn', 'WaitNextRound'].includes(battle.state) &&
      Object.values(battle.pairs).length > 0 &&
      rivals.length > 0 &&
      currentPairIdx >= 0
  )

  const gameIsOver = battle?.state === 'GameIsOver' && battle?.currentWinner

  return (
    <>
      {battle?.state === 'Registration' &&
        (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {gameIsOn && <BattleRound />}
      {gameIsOver && <BattleWinner battle={battle} />}
      {battle && Object.keys(battle.players).length > 0 && (
        <BattlePlayersQueue />
      )}
    </>
  )
}

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
      ['GameIsOn', 'WaitNextRound'].includes(battle.status) &&
      // Object.values(battle.pairs).length > 0 &&
      rivals.length > 0 &&
      currentPairIdx >= 0
  )

  const gameIsOver = battle?.status === 'GameIsOver' //&& battle?.currentWinner

  return (
    <>
      {/*{battle?.status === 'Registration' &&*/}
      {/*  (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}*/}
      {/*{gameIsOn && <BattleRound />}*/}
      {/*{gameIsOver && <BattleWinner battle={battle} />}*/}
      {battle && Object.keys(battle.heroes).length > 0 && (
        <BattlePlayersQueue />
      )}
    </>
  )
}

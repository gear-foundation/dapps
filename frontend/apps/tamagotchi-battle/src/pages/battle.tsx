import { BattlePlayersQueue } from 'features/battle/components/battle-players-queue';
import { BattleWaitRegistration } from 'features/battle/components/battle-wait-registration';
import { useBattle } from 'features/battle/context';
import { BattleWaitAdmin } from 'features/battle/components/battle-wait-admin';
import { BattleRound } from 'features/battle/components/battle-round';
import { BattleWinner } from 'features/battle/components/battle-winner';

export const Battle = () => {
  const { battle, rivals, currentPairIdx, isAdmin } = useBattle();

  const gameIsOn = Boolean(
    battle &&
      ['GameIsOn', 'WaitNextRound'].includes(battle.state) &&
      Object.values(battle.pairs).length > 0 &&
      rivals.length > 0 &&
      currentPairIdx >= 0,
  );

  const gameIsOver = battle?.state === 'GameIsOver' && battle?.currentWinner;
  console.log('BBBB');
  console.log(battle);
  console.log(!!gameIsOver);
  return (
    <>
      {battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {gameIsOn && <BattleRound />}
      {gameIsOver && <BattleWinner battle={battle} />}
      {battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}
    </>
  );
};

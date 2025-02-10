import { BattlePlayersQueue } from '@/features/battle/components/battle-players-queue';
import { BattleRound } from '@/features/battle/components/battle-round';
import { BattleWaitAdmin } from '@/features/battle/components/battle-wait-admin';
import { BattleWaitRegistration } from '@/features/battle/components/battle-wait-registration';
import { BattleWinner } from '@/features/battle/components/battle-winner';
import { useBattle } from '@/features/battle/context';

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

  return (
    <>
      {battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {gameIsOn && <BattleRound />}
      {gameIsOver && <BattleWinner battle={battle} />}
      {battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}
    </>
  );
};

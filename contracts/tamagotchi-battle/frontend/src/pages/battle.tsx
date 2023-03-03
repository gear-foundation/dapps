import { BattlePlayersQueue } from 'components/sections/battle-players-queue';
import { BattleWaitRegistration } from 'components/sections/battle-wait-registration';
import { useApp, useBattle } from 'app/context';
import { BattleWaitAdmin } from 'components/sections/battle-wait-admin';
import { BattleRound } from 'components/sections/battle-round';
import { BattleWinner } from 'components/sections/battle-winner';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battle, rivals, currentPairIdx } = useBattle();

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

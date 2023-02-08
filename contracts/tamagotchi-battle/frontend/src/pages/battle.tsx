import { BattlePlayersQueue } from '../components/sections/battle-players-queue';
import { BattleWaitRegistration } from '../components/sections/battle-wait-registration';
import { useApp, useBattle } from '../app/context';
import { BattleWaitAdmin } from '../components/sections/battle-wait-admin';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battleState: battle } = useBattle();

  return (
    <>
      {battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}
    </>
  );
};

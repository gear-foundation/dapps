import { BattlePlayersQueue } from 'components/sections/battle-players-queue';
import { BattleWaitRegistration } from 'components/sections/battle-wait-registration';
import { useApp, useBattle } from 'app/context';
import { BattleWaitAdmin } from 'components/sections/battle-wait-admin';
import { BattleRound } from 'components/sections/battle-round';
import { BattleWinner } from 'components/sections/battle-winner';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battle, rivals, currentPlayer } = useBattle();

  return (
    <section className="grid grid-rows-[1fr_auto_auto] h-[calc(100vh-216px)]" >
      <div className="flex flex-col items-center gap-9 text-center w-full">
        This is Launche Page
      </div>

      {battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}
      {battle && ['GameIsOn', 'WaitNextRound'].includes(battle.state) && rivals.length && <BattleRound />}
      {battle && battle?.state === 'GameIsOver' && rivals.length && currentPlayer && <BattleWinner battle={battle} />}
      {battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}
    </section>
  );
};

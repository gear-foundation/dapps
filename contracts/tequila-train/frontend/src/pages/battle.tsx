import { useApp, useBattle } from 'app/context';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battle, rivals, currentPlayer } = useBattle();

  return (
    <>
      {/*{battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}*/}
      {/*{battle && ['GameIsOn', 'WaitNextRound'].includes(battle.state) && rivals.length && <BattleRound />}*/}
      {/*{battle && battle?.state === 'GameIsOver' && rivals.length && currentPlayer && <BattleWinner battle={battle} />}*/}
      {/*{battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}*/}
    </>
  );
};

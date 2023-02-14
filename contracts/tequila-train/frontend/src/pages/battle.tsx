import { useApp, useGame } from 'app/context';

export const Battle = () => {
  const { isAdmin } = useApp();
  const { battle, rivals, currentPlayer } = useGame();

  return (
    <>
      {/*{battle?.state === 'Registration' && (isAdmin ? <BattleWaitAdmin /> : <BattleWaitRegistration />)}*/}
      {/*{battle && ['GameIsOn', 'WaitNextRound'].includes(battle.state) && rivals.length && <BattleRound />}*/}
      {/*{battle && battle?.state === 'GameIsOver' && rivals.length && currentPlayer && <BattleWinner battle={battle} />}*/}
      {/*{battle && Object.keys(battle.players).length > 0 && <BattlePlayersQueue />}*/}
    </>
  );
};

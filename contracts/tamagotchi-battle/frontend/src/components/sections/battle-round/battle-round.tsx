import { BattleRoundInfo } from '../battle-round-info';
import { BattleRoundStats } from '../battle-round-stats';
import { BattleRoundPlayers } from '../battle-round-players';

export const BattleRound = () => {
  return (
    <section className="container grow flex flex-col">
      <BattleRoundStats />
      <BattleRoundPlayers />
      <BattleRoundInfo />
    </section>
  );
};

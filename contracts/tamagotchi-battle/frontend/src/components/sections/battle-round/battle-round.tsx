import { BattleRoundInfo } from 'components/sections/battle-round-info';
import { BattleRoundStats } from 'components/sections/battle-round-stats';
import { BattleRoundPlayers } from 'components/sections/battle-round-players';

export const BattleRound = () => {
  return (
    <section className="container grow flex flex-col">
      <BattleRoundStats />
      <BattleRoundPlayers />
      <BattleRoundInfo />
    </section>
  );
};

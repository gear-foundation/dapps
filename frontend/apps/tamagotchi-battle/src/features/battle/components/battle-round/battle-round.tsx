import { BattleRoundInfo } from '../battle-round-info';
import { BattleRoundPlayers } from '../battle-round-players';
import { BattleRoundStats } from '../battle-round-stats';
import { BattleTableChampions } from '../battle-table-champions';
import { BattleTablePairs } from '../battle-table-pairs';

export const BattleRound = () => {
  return (
    <>
      <section className="container grow flex flex-col">
        <BattleRoundStats />
        <BattleRoundPlayers />
        <BattleRoundInfo />
      </section>

      <BattleTablePairs />
      <BattleTableChampions />
    </>
  );
};

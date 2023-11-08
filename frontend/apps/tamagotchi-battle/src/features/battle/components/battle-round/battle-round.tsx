import { BattleRoundInfo } from '../battle-round-info';
import { BattleRoundStats } from '../battle-round-stats';
import { BattleRoundPlayers } from '../battle-round-players';
import { BattleTablePairs } from '../battle-table-pairs';
import { BattleTableChampions } from '../battle-table-champions';

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

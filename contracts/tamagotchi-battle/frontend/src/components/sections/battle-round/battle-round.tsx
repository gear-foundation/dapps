import { BattleRoundInfo } from 'components/sections/battle-round-info';
import { BattleRoundStats } from 'components/sections/battle-round-stats';
import { BattleRoundPlayers } from 'components/sections/battle-round-players';
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

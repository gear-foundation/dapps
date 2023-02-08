import { BattleStateResponse } from '../../../app/types/battles';
import { BattleRoundInfo } from '../battle-round-info';
import { BattleRoundStats } from '../battle-round-stats';
import { BattleRoundAvatars } from '../battle-round-avatars';

export const BattleRound = ({ battle }: { battle: BattleStateResponse }) => {
  return (
    <section className="container grow">
      <BattleRoundStats battle={battle} />
      <BattleRoundAvatars />
      <BattleRoundInfo />
    </section>
  );
};

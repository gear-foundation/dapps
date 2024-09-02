import { Button } from '@gear-js/ui';
import { useAccount } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { Content } from 'components';
import { DashboardProps } from 'types';
import { useLotteryMessage } from 'hooks';
import { STATUS, SUBHEADING } from 'consts';
import { Dashboard } from './dashboard';
import { Players } from './players';
import { PlayerStatus } from './player-status';

type Props = {
  isOwner: boolean;
  dashboard: DashboardProps;
  prizeFund: string;
  players: HexString[];
  cost: string;
  onResetButtonClick: () => void;
};

function Pending({ isOwner, dashboard, prizeFund, players, cost, onResetButtonClick }: Props) {
  const { account } = useAccount();
  const sendMessage = useLotteryMessage();
  const pickWinner = () => sendMessage({ payload: { PickWinner: null }, value: prizeFund });

  const { startTime, endTime, status, winner, countdown } = dashboard;
  const subheading = winner ? `Uhhu! ${winner} is the winner!` : SUBHEADING.PENDING;
  const isLotteryActive = status === STATUS.PENDING;
  const isPlayerStatus = !isOwner && winner;
  const isPlayerWinner = winner === account?.decodedAddress;
  const isAnyPlayer = players.length > 0;

  return (
    <Content subheading={subheading}>
      {isOwner &&
        (winner || (!isLotteryActive && !isAnyPlayer) ? (
          <Button text="Start new Game" onClick={onResetButtonClick} />
        ) : (
          <Button text="Pick random winner" disabled={isLotteryActive} onClick={pickWinner} />
        ))}
      <Dashboard startTime={startTime} endTime={endTime} status={status} winner={winner} countdown={countdown} />
      {isPlayerStatus && <PlayerStatus isWinner={isPlayerWinner} />}
      <Players list={players} balance={cost} />
    </Content>
  );
}

export { Pending };

import { Hex } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { Content, Loader } from 'components';
import { useLottery, useLotteryStatus } from 'hooks';
import { getDate, getNumber, isWinner } from 'utils';
import { STATUS, SUBHEADING } from 'consts';
import { OwnerStart } from './owner-start';
import { PlayerStart } from './player-start';
import { Pending } from './pending';

function Home() {
  const { account } = useAccount();

  const { lottery, isLotteryRead } = useLottery();
  const { admin, started, ending, ftActorId } = lottery || {};

  const cost = lottery?.participationCost || '';
  const prizeFund = lottery?.prizeFund || '';
  const participationCost = lottery?.participationCost || '';
  const players = lottery ? Object.values(lottery.players) : [];
  const winner = lottery && isWinner(lottery.winner) ? lottery.winner : ('' as Hex);
  const isOwner = account?.decodedAddress === admin;
  const isPlayer = players.some((playerId) => playerId === account?.decodedAddress);
  const isParticipant = isPlayer || isOwner;

  const startTime = getNumber(started || '');
  const endTime = getNumber(ending || '');

  const { status, countdown, resetStatus } = useLotteryStatus(endTime);
  const isLotteryStarted = status !== STATUS.AWAIT;
  const isLotteryActive = status === STATUS.PENDING;
  const dashboard = { startTime: getDate(startTime), endTime: getDate(endTime), status, winner, countdown };

  return isLotteryRead ? (
    <>
      {isLotteryStarted && isParticipant && (
        <Pending
          isOwner={isOwner}
          dashboard={dashboard}
          prizeFund={prizeFund}
          players={players}
          participationCost={participationCost}
          onResetButtonClick={resetStatus}
        />
      )}
      {!isLotteryStarted && isOwner && <OwnerStart />}
      {isLotteryActive && !isParticipant && <PlayerStart cost={cost} isToken={!!ftActorId} />}
      {!isLotteryActive && !isParticipant && <Content subheading={SUBHEADING.AWAIT} />}
    </>
  ) : (
    <Loader />
  );
}

export { Home };

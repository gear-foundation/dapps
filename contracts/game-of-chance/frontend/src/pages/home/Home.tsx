import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { Content, Loader } from 'components';
import { useLotteryState, useLotteryStatus } from 'hooks';
import { getDate, isWinner } from 'utils';
import { STATUS, SUBHEADING } from 'consts';
import { OwnerStart } from './owner-start';
import { PlayerStart } from './player-start';
import { Pending } from './pending';

function Home() {
  const { account } = useAccount();

  const { state, isStateRead } = useLotteryState();
  const { admin, started, ending, fungibleToken } = state || {};

  const startTime = +withoutCommas(started || '');
  const endTime = +withoutCommas(ending || '');
  const cost = state?.participationCost || '';
  const prizeFund = state?.prizeFund || '';
  const players = state?.players || [];
  const winner =
    state && isWinner(state.winner) ? state.winner : ('' as HexString);
  const isOwner = account?.decodedAddress === admin;
  const isPlayer = players.some(
    (playerId) => playerId === account?.decodedAddress
  );
  const isParticipant = isPlayer || isOwner;

  const { status, countdown, resetStatus } = useLotteryStatus(endTime);
  const isLotteryStarted = status !== STATUS.AWAIT;
  const isLotteryActive = status === STATUS.PENDING;

  const dashboard = {
    startTime: getDate(startTime),
    endTime: getDate(endTime),
    status,
    winner,
    countdown,
  };

  return isStateRead ? (
    <>
      {isLotteryStarted && isParticipant && (
        <Pending
          isOwner={isOwner}
          dashboard={dashboard}
          prizeFund={prizeFund}
          players={players}
          cost={cost}
          onResetButtonClick={resetStatus}
        />
      )}

      {!isLotteryStarted && isOwner && <OwnerStart />}
      {isLotteryActive && !isParticipant && (
        <PlayerStart cost={cost} isToken={!!fungibleToken} />
      )}
      {!isLotteryActive && !isParticipant && (
        <Content subheading={SUBHEADING.AWAIT} />
      )}
    </>
  ) : (
    <Loader />
  );
}

export { Home };

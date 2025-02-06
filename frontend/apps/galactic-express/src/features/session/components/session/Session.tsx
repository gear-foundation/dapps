import { Button } from '@gear-js/ui';
import { CSSProperties, useState } from 'react';
import clsx from 'clsx';
import { getVaraAddress, withoutCommas } from '@gear-js/react-hooks';
import { HexString } from '@gear-js/api';
import { Container } from '@/components';
import LeftDoubleArrowSVG from '../../assets/left-double-arrow.svg?react';
import LeftArrowSVG from '../../assets/left-arrow.svg?react';
import { PLAYER_COLORS } from '../../consts';
import { Event, Rank, Session as SessionType, Turns, Participant, RankWithName } from '../../types';
import { Traits } from '../traits';
import { Radar } from '../radar';
import { Table } from '../table';
import styles from './Session.module.scss';

type Props = {
  session: SessionType;
  turns: Turns;
  rankings: Rank[];
  userId?: HexString;
  participants: Participant[];
  admin: string | undefined;
};

function Session({ session, turns, rankings, userId, participants, admin }: Props) {
  const { altitude, weather, reward } = session;
  const roundsCount = turns.length;

  const [roundIndex, setRoundIndex] = useState(0);
  const roundNumber = roundIndex + 1;
  const isFirstPage = roundNumber === 1;
  const isLastPage = roundNumber === roundsCount;

  const nextPage = () => setRoundIndex((prevValue) => prevValue + 1);
  const prevPage = () => setRoundIndex((prevValue) => prevValue - 1);
  const firstPage = () => setRoundIndex(0);
  const lastPage = () => setRoundIndex(roundsCount - 1);

  const defineFuelLeftFormat = (isAlive: boolean, fuelLeft: number) => {
    return isAlive && fuelLeft ? String(fuelLeft) : ' - ';
  };

  const getEvents = (): Event[] =>
    turns[roundIndex]
      .slice()
      .sort((a, b) => {
        const indexA = participants.findIndex((p) => p[0] === a[0]);
        const indexB = participants.findIndex((p) => p[0] === b[0]);

        return indexA - indexB;
      })
      ?.map((participantInfo) => {
        const isAlive = 'alive' in participantInfo[1];
        const firstDeadRound = turns.findIndex((turn) => {
          const part = turn.find((participant) => participant[0] === participantInfo[0]) || [];

          return Object.keys(part[1] || {})[0] !== 'alive';
        });

        return {
          participant: participantInfo[0],
          name: participants.find((part) => part[0] === participantInfo[0])?.[1].name,
          deadRound: !isAlive,
          firstDeadRound,
          fuelLeft: defineFuelLeftFormat(
            isAlive,
            'alive' in participantInfo[1] ? participantInfo[1]?.alive?.fuel_left : 0,
          ),
          payload: 'alive' in participantInfo[1] ? String(participantInfo[1].alive.payload_amount) : ' - ',
          lastAltitude: String(
            Math.round(
              Number(withoutCommas(altitude)) /
                (firstDeadRound !== -1 && firstDeadRound < roundNumber
                  ? roundsCount - firstDeadRound
                  : roundsCount - roundNumber + 1),
            ),
          ),
          haltReason: 'alive' in participantInfo[1] ? null : participantInfo[1].destroyed,
        };
      });

  const getFeedItems = () =>
    getEvents()?.map(({ participant, payload, lastAltitude, fuelLeft, deadRound, haltReason }, index) => (
      <li key={participant} className={styles.item} style={{ '--color': PLAYER_COLORS[index] } as CSSProperties}>
        <h3 className={styles.heading}>{getVaraAddress(participant)}</h3>
        <div className={styles.bodyItem}>
          <p className={styles.text}>Data:</p>
          <p className={styles.text}>Alive:</p>
          <p className={styles.textValue}>{String(!deadRound)},</p>
          <p className={styles.text}>Fuel Left:</p>
          <p className={styles.textValue}>{fuelLeft},</p>
          <p className={styles.text}>Last Altitude:</p>
          <p className={styles.textValue}>{lastAltitude},</p>
          <p className={styles.text}>Payload:</p>
          <p className={styles.textValue}>{payload},</p>
          <p className={styles.text}>Halt:</p>
          <p className={styles.textValue}>{haltReason || 'null'},</p>
        </div>
      </li>
    ));

  const sortRanks = () => {
    const isAllZeros = rankings.every((rank) => rank[1] === 0);

    const sortedRanks = isAllZeros
      ? []
      : rankings.sort((rankA, rankB) => (Number(rankA[1]) < Number(rankB[1]) ? 1 : -1));

    return sortedRanks;
  };

  const defineWinners = () => {
    const sortedRanks = sortRanks();
    const highestRank = sortedRanks?.[0]?.[1];

    const winners = sortedRanks
      .filter((item) => item[1] === highestRank)
      .map((item) => [...item, participants.find((part) => part[0] === item[0])?.[1].name || '']) as RankWithName[];

    return {
      isUserWinner: winners.map((item) => item[0]).includes(userId || '0x'),
      userRank: sortedRanks.find((item) => item[0] === userId)?.[1] || '0',
      winners,
    };
  };

  const definedWinners = defineWinners();

  return (
    <div className={styles.container}>
      <div
        className={clsx(
          styles.courtain,
          definedWinners.winners.map((item) => item[0]).includes(userId || '0x')
            ? styles.courtainGreen
            : styles.courtainRed,
        )}
      />

      <Container>
        <header className={styles.header}>
          <h2 className={styles.heading}>Session</h2>

          <div className={styles.navigation}>
            <Button icon={LeftDoubleArrowSVG} color="transparent" onClick={firstPage} disabled={isFirstPage} />
            <Button icon={LeftArrowSVG} color="transparent" onClick={prevPage} disabled={isFirstPage} />

            <p>
              Turn {roundNumber} of {roundsCount}
            </p>

            <Button
              icon={LeftArrowSVG}
              color="transparent"
              onClick={nextPage}
              className={styles.rotatedArrow}
              disabled={isLastPage}
            />
            <Button
              icon={LeftDoubleArrowSVG}
              color="transparent"
              onClick={lastPage}
              className={styles.rotatedArrow}
              disabled={isLastPage}
            />
          </div>
        </header>

        <div className={styles.body}>
          <Table data={getEvents()} userId={userId} />

          <Traits altitude={altitude} weather={weather} reward={reward} />

          <ul className={styles.feed}>{getFeedItems()}</ul>
        </div>
      </Container>

      <Radar
        currentEvents={getEvents()}
        currentRound={roundIndex}
        roundsCount={roundsCount}
        isWinner={definedWinners.isUserWinner}
        winners={definedWinners.winners}
        userRank={String(definedWinners.userRank)}
        admin={admin}
      />
    </div>
  );
}

export { Session };

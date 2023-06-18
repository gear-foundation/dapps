import { Button } from '@gear-js/ui';
import { CSSProperties, useState } from 'react';
import { Container } from 'components';
import { ReactComponent as LeftDoubleArrowSVG } from '../../assets/left-double-arrow.svg';
import { ReactComponent as LeftArrowSVG } from '../../assets/left-arrow.svg';
import { PLAYER_COLORS } from '../../consts';
import { LaunchState, Session as SessionType } from '../../types';
import { Traits } from '../traits';
import { Radar } from '../radar';
import { Table } from '../table/Table';
import styles from './Session.module.scss';

type Props = {
  id: string;
  session: SessionType;
  events: LaunchState['events'];
};

function Session({ id, session, events }: Props) {
  const { altitude, weather, fuelPrice, reward } = session;
  const roundsCount = Object.keys(events).length;

  const [roundIndex, setRoundIndex] = useState(0);
  const roundNumber = roundIndex + 1;
  const isFirstPage = roundNumber === 1;
  const isLastPage = roundNumber === roundsCount;
  const currentEvents = events[roundIndex];

  const nextPage = () => setRoundIndex((prevValue) => prevValue + 1);
  const prevPage = () => setRoundIndex((prevValue) => prevValue - 1);
  const firstPage = () => setRoundIndex(0);
  const lastPage = () => setRoundIndex(roundsCount - 1);

  const getFeedItems = () =>
    currentEvents.map(({ participant, halt }, index) =>
      halt ? (
        <li key={participant} className={styles.item} style={{ '--style': PLAYER_COLORS[index] } as CSSProperties}>
          <h3 className={styles.heading}>{participant}</h3>
          <p className={styles.text}>{halt.split(/(?=[A-Z])/).join(' ')}</p>
        </li>
      ) : null,
    );

  return (
    <div className={styles.container}>
      <Container>
        <header className={styles.header}>
          <h2 className={styles.heading}>Session #{id}</h2>

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
          <Table data={currentEvents} />

          <Traits altitude={altitude} weather={weather} fuelPrice={fuelPrice} reward={reward} />

          <ul className={styles.feed}>{getFeedItems()}</ul>
        </div>
      </Container>

      <Radar currentEvents={currentEvents} currentRound={roundIndex} roundsCount={roundsCount} />
    </div>
  );
}

export { Session };

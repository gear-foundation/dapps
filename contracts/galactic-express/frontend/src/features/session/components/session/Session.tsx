import { Button, Input } from '@gear-js/ui';
import { CSSProperties, useState } from 'react';
import { Container } from 'components';
import { ReactComponent as LeftDoubleArrowSVG } from '../../assets/left-double-arrow.svg';
import { ReactComponent as LeftArrowSVG } from '../../assets/left-arrow.svg';
import { PLAYER_COLORS } from '../../consts';
import { LaunchState, Session as SessionType } from '../../types';
import { Table } from '../table';
import { Traits } from '../traits';
import styles from './Session.module.scss';
import { Radar } from '../radar';

type Props = {
  id: string;
  session: SessionType;
  events: LaunchState['events'];
};

function Session({ id, session, events }: Props) {
  const { altitude, weather, fuelPrice, reward } = session;
  const roundsCount = Object.keys(events).length;

  const [pageIndex, setPageIndex] = useState(0);
  const page = pageIndex + 1;
  const isFirstPage = page === 1;
  const isLastPage = page === roundsCount;
  const currentEvents = events[pageIndex];

  const nextPage = () => setPageIndex((prevValue) => prevValue + 1);
  const prevPage = () => setPageIndex((prevValue) => prevValue - 1);
  const firstPage = () => setPageIndex(0);
  const lastPage = () => setPageIndex(roundsCount - 1);

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

            <div className={styles.inputWrapper}>
              <Input label="turn" className={styles.input} value={page} onChange={() => {}} />
              <span className={styles.total}>of {roundsCount}</span>
            </div>

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

      <Radar currentEvents={currentEvents} currentRound={pageIndex} roundsCount={roundsCount} />
    </div>
  );
}

export { Session };

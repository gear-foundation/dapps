import { Button, Input } from '@gear-js/ui';
import { Container } from 'components';
import { useState } from 'react';
import { ReactComponent as LeftDoubleArrowSVG } from '../../assets/left-double-arrow.svg';
import { ReactComponent as LeftArrowSVG } from '../../assets/left-arrow.svg';
import { useLaunchState } from '../../hooks';
import { Table } from '../table';
import styles from './Session.module.scss';

function Session() {
  const state = useLaunchState();
  const { currentSession, events } = state || {};
  const pagesCount = Object.keys(events || {}).length;

  const [pageIndex, setPageIndex] = useState(0);
  const page = pageIndex + 1;
  const currentEvents = events?.[pageIndex];

  const nextPage = () => setPageIndex((prevValue) => prevValue + 1);
  const prevPage = () => setPageIndex((prevValue) => prevValue - 1);
  const firstPage = () => setPageIndex(0);
  const lastPage = () => setPageIndex(pagesCount - 1);

  return (
    <Container>
      <header className={styles.header}>
        <h2 className={styles.heading}>Session #{1}</h2>

        <div className={styles.navigation}>
          <Button icon={LeftDoubleArrowSVG} color="transparent" onClick={firstPage} />
          <Button icon={LeftArrowSVG} color="transparent" onClick={prevPage} />

          <div className={styles.inputWrapper}>
            <Input label="turn" className={styles.input} value={page} onChange={() => {}} />
            <span className={styles.total}>of {pagesCount}</span>
          </div>

          <Button icon={LeftArrowSVG} color="transparent" onClick={nextPage} className={styles.rotatedArrow} />
          <Button icon={LeftDoubleArrowSVG} color="transparent" onClick={lastPage} className={styles.rotatedArrow} />
        </div>
      </header>

      {currentEvents && <Table data={currentEvents} />}
    </Container>
  );
}

export { Session };

import { Fragment } from 'react';
import clsx from 'clsx';
import { TABLE_HEADINGS } from 'features/session/consts';
import { ReactComponent as CheckSVG } from '../../assets/check.svg';
import { ReactComponent as CrossSVG } from '../../assets/cross.svg';
import { Event } from '../../types';
import styles from './Table.module.scss';

type Props = {
  data: Event[];
};

function Table({ data }: Props) {
  const getHeader = () =>
    TABLE_HEADINGS.map((text) => (
      <div key={text} className={styles.headerCell}>
        {text}
      </div>
    ));

  const getBody = () =>
    data.map(({ participant, deadRound, fuelLeft, lastAltitude, payload, halt }) => (
      <Fragment key={participant}>
        <div className={clsx(styles.bodyCell, styles.firstColumn)}>{participant}</div>
        <div className={styles.bodyCell}>{deadRound ? <CrossSVG /> : <CheckSVG />}</div>
        <div className={styles.bodyCell}>{fuelLeft}</div>
        <div className={styles.bodyCell}>{lastAltitude}</div>
        <div className={styles.bodyCell}>{payload}</div>
        <div className={clsx(styles.bodyCell, styles.lastColumn)}>{halt ? halt.split(/(?=[A-Z])/).join(' ') : '-'}</div>
      </Fragment>
    ));

  return (
    <div className={styles.table}>
      {getHeader()}
      {getBody()}
    </div>
  );
}

export { Table };

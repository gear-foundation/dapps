import { CSSProperties, Fragment } from 'react';
import clsx from 'clsx';
import { ReactComponent as CheckSVG } from '../../assets/check.svg';
import { ReactComponent as CrossSVG } from '../../assets/cross.svg';
import { PLAYER_COLORS, TABLE_HEADINGS } from '../../consts';
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
    data.map(({ participant, deadRound, fuelLeft, lastAltitude, payload, halt }, index) => (
      <Fragment key={participant}>
        <div
          className={clsx(styles.bodyCell, styles.firstColumn)}
          style={{ '--color': PLAYER_COLORS[index] } as CSSProperties}>
          {participant}
        </div>
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

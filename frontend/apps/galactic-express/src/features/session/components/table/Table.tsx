import { CSSProperties, Fragment } from 'react';
import { cx } from 'utils';
import { shortenString } from 'features/session/utils';
import { ReactComponent as CheckSVG } from '../../assets/check.svg';
import { ReactComponent as CrossSVG } from '../../assets/cross.svg';
import { PLAYER_COLORS, TABLE_HEADINGS } from '../../consts';
import { Event } from '../../types';
import styles from './Table.module.scss';

type Props = {
  data: Event[];
  userId?: string;
};

function Table({ data, userId }: Props) {
  const getHeader = () =>
    TABLE_HEADINGS.map((text) => (
      <div key={text} className={styles.headerCell}>
        {text}
      </div>
    ));

  const getBody = () =>
    data?.map(({ participant, name, deadRound, fuelLeft, lastAltitude, payload }, index) => (
      <Fragment key={participant}>
        <div
          className={cx(styles.bodyCell, styles.firstColumn)}
          style={{ '--color': PLAYER_COLORS[index] } as CSSProperties}>
          <span>
            {shortenString(participant, 4)}{' '}
            {userId === participant ? <span className={cx(styles.yourAddressSpan)}> (You)</span> : ''}
          </span>
        </div>
        <div className={styles.bodyCell}>{name}</div>
        <div className={styles.bodyCell}>{deadRound ? <CrossSVG /> : <CheckSVG />}</div>
        <div className={styles.bodyCell}>{fuelLeft}</div>
        <div className={styles.bodyCell}>{lastAltitude}</div>
        <div className={cx(styles.bodyCell, styles.lastColumn)}>{payload}</div>
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

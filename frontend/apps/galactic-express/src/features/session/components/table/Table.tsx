import { CSSProperties, Fragment } from 'react';
import { cx } from '@/utils';
import { shortenString } from '@/features/session/utils';
import { getVaraAddress } from '@gear-js/react-hooks';
import CheckSVG from '../../assets/check.svg?react';
import CrossSVG from '../../assets/cross.svg?react';
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
    data?.map(({ participant, name, deadRound, fuelLeft, lastAltitude, payload, haltReason }, index) => (
      <Fragment key={participant}>
        <div
          className={cx(styles.bodyCell, styles.firstColumn)}
          style={{ '--color': PLAYER_COLORS[index] } as CSSProperties}>
          <span>
            {shortenString(getVaraAddress(participant), 4)}{' '}
            {userId === participant ? <span className={cx(styles.yourAddressSpan)}> (You)</span> : ''}
          </span>
        </div>
        <div className={styles.bodyCell}>{name}</div>
        <div className={styles.bodyCell}>{deadRound ? <CrossSVG /> : <CheckSVG />}</div>
        <div className={styles.bodyCell}>{fuelLeft}</div>
        <div className={styles.bodyCell}>{lastAltitude}</div>
        <div className={styles.bodyCell}>{payload}</div>
        <div className={cx(styles.bodyCell, styles.lastColumn)}>{haltReason || ' - '}</div>
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

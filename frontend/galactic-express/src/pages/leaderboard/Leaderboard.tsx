import { useLaunchState } from 'features/session';
import { Fragment, useEffect } from 'react';
import clsx from 'clsx';
import { Loader } from 'components';
import { withoutCommas } from '@gear-js/react-hooks';
import styles from './Leaderboard.module.scss';

const TABLE_HEADINGS = ['Rank', 'Address', 'Balance', 'Score'];

function Leaderboard() {
  const state = useLaunchState();
  const { participants } = state || {};

  const getHeader = () =>
    TABLE_HEADINGS.map((text) => (
      <div key={text} className={styles.headerCell}>
        {text}
      </div>
    ));

  const getBody = () =>
    Object.entries(participants || {})
      .sort(([, player], [, nextPlayer]) => (+withoutCommas(player.score) > +withoutCommas(nextPlayer.score) ? -1 : 1))
      .map(([address, { balance, score }], index) => (
        <Fragment key={address}>
          <div className={clsx(styles.bodyCell, styles.firstColumn)}>{index + 1}</div>
          <div className={styles.bodyCell}>{address}</div>
          <div className={styles.bodyCell}>{balance}</div>
          <div className={clsx(styles.bodyCell, styles.lastColumn)}>{score}</div>
        </Fragment>
      ));

  useEffect(() => {
    document.body.classList.add('leaderboard');

    return () => {
      document.body.classList.remove('leaderboard');
    };
  }, []);

  return state ? (
    <>
      <h2 className={styles.heading}>Leaderboard</h2>

      <div className={styles.table}>
        {getHeader()}
        {getBody()}
      </div>
    </>
  ) : (
    <Loader />
  );
}

export { Leaderboard };

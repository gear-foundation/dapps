import { useSetAtom } from 'jotai';
import { cx } from 'utils';
import { REGISTRATION_STATUS } from 'atoms';
import { getVaraAddress, useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useLaunchMessage } from 'features/session/hooks';
import { shortenString } from 'features/session/utils';
import { RankWithName } from 'features/session/types';
import styles from './WinStatus.module.scss';

type Props = {
  type: 'win' | 'lose';
  userRank: string;
  winners: RankWithName[];
  admin: string | undefined;
};

function WinStatus({ type, userRank, winners, admin }: Props) {
  const { meta, message: sendNewSessionMessage } = useLaunchMessage();
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);
  const { account } = useAccount();

  const isAdmin = admin === account?.decodedAddress;

  const onInBlock = () => {
    setRegistrationStatus('registration');
  };

  const handleCreateNewSession = () => {
    if (!meta) {
      return;
    }

    if (isAdmin) {
      sendNewSessionMessage({ payload: { CancelGame: null }, onInBlock });
    } else {
      sendNewSessionMessage({ payload: { LeaveGame: null }, onInBlock });
    }
  };

  return (
    <div className={cx(styles.container, styles[type])}>
      <h2 className={cx(styles.title, styles[`title-${type}`])}>{type === 'win' ? 'You Win' : 'Game Is Over'}</h2>
      <div className={cx(styles.ranks)}>
        <span className={cx(styles.prize)}>
          Your Rank: <span className={cx(styles.rank)}>{userRank}</span>
        </span>
      </div>
      {winners.length && (
        <div className={cx(styles.winners)}>
          Winners:{' '}
          <ul>
            {winners.map((item) => (
              <li className={cx(account?.decodedAddress === item[0] ? styles['user-winner'] : '')}>
                {item[2] || shortenString(getVaraAddress(item[0]), 6)}
              </li>
            ))}
          </ul>
        </div>
      )}
      <Button
        text={isAdmin ? 'Play again' : 'Leave game'}
        className={cx(styles.btn)}
        onClick={handleCreateNewSession}
        color="lightGreen"
      />
    </div>
  );
}

export { WinStatus };

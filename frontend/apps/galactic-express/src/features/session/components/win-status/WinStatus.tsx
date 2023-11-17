import { useAtomValue } from 'jotai';
import { CURRENT_CONTRACT_ADDRESS_ATOM } from 'atoms';
import { cx } from 'utils';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useNewSessionMessage } from 'features/session/hooks';
import { shortenString } from 'features/session/utils';
import { Rank } from 'features/session/types';
import styles from './WinStatus.module.scss';

type Props = {
  type: 'win' | 'lose';
  userRank: string;
  winners: Rank[];
};

function WinStatus({ type, userRank, winners }: Props) {
  const contractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const { meta, message: sendNewSessionMessage } = useNewSessionMessage(contractAddress);
  const { account } = useAccount();

  const handleCreateNewSession = () => {
    if (meta) {
      sendNewSessionMessage({ payload: { CreateNewSession: null } });
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
                {shortenString(item[0], 6)}
              </li>
            ))}
          </ul>
        </div>
      )}
      <Button text="Play again" className={cx(styles.btn)} onClick={handleCreateNewSession} color="lightGreen" />
    </div>
  );
}

export { WinStatus };

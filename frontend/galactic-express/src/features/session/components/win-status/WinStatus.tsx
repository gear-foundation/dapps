import { useAtomValue } from 'jotai';
import { CURRENT_CONTRACT_ADDRESS_ATOM } from 'atoms';
import { cx } from 'utils';
import { Button } from '@gear-js/ui';
import { useNewSessionMessage } from 'features/session/hooks';
import styles from './WinStatus.module.scss';

type Props = {
  type: 'win' | 'lose';
  userRank: number;
};

function WinStatus({ type, userRank }: Props) {
  const contractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const { meta, message: sendNewSessionMessage } = useNewSessionMessage(contractAddress);

  const handleCreateNewSession = () => {
    if (meta) {
      sendNewSessionMessage({ CreateNewSession: null });
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

      <Button text="Play again" className={cx(styles.btn)} onClick={handleCreateNewSession} />
    </div>
  );
}

export { WinStatus };

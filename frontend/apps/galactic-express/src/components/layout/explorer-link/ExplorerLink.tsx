import { useAccount } from '@gear-js/react-hooks';
import { cx } from 'utils';
import styles from './ExplorerLink.module.scss';
import { ReactComponent as UserSearch } from './assets/icons/user-search.svg';

function ExplorerLink() {
  const { account } = useAccount();

  return (
    <a href={` https://vara.subscan.io/account/${account?.address}`} target="_blank" rel="noreferrer">
      <div className={styles.container}>
        <UserSearch />
        <span className={cx(styles.text)}>View in Blockchain Explorer</span>
      </div>
    </a>
  );
}

export { ExplorerLink };

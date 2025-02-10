import { useAccount } from '@gear-js/react-hooks';

import Signless from '@/assets/icons/signless.svg?react';

import { useSignlessTransactions } from '../../context';

import styles from './signless-active.module.css';

function SignlessActive() {
  const { account } = useAccount();
  const { session, isSessionReady } = useSignlessTransactions();

  return account && isSessionReady && session ? (
    <div className={styles.container}>
      <Signless />
      <span className={styles.text}>Signless Session</span>
    </div>
  ) : null;
}

export { SignlessActive };

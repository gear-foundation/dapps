import clsx from 'clsx';
import { AnimatePresence, motion } from 'framer-motion';

import styles from './zk-verification.module.scss';

const variants = {
  enter: {
    opacity: 0,
    y: 10,
    scale: 0.98,
  },
  center: {
    zIndex: 1,
    opacity: 1,
    y: 0,
    scale: 1,
  },
  exit: {
    zIndex: 0,
    opacity: 0,
    y: -10,
    scale: 0.98,
  },
};

type Props = {
  isWaitingShuffleVerification?: boolean;
  isWaitingPartialDecryptionsForPlayersCards?: boolean;
  isWaitingTableCards?: boolean;
  isWaitingForCardsToBeDisclosed?: boolean;
  isWaitingForAllTableCardsToBeDisclosed?: boolean;
  isInLoader?: boolean;
};

export function ZkVerification({
  isWaitingShuffleVerification,
  isWaitingPartialDecryptionsForPlayersCards,
  isWaitingTableCards,
  isWaitingForCardsToBeDisclosed,
  isWaitingForAllTableCardsToBeDisclosed,
  isInLoader,
}: Props) {
  const getAction = () => {
    if (isWaitingShuffleVerification) {
      return 'Shuffle';
    }
    if (isWaitingPartialDecryptionsForPlayersCards) {
      return 'Partial Decryptions for Players Cards';
    }
    if (isWaitingTableCards) {
      return 'Table Cards';
    }
    if (isWaitingForCardsToBeDisclosed) {
      return 'Cards to be Disclosed';
    }
    if (isWaitingForAllTableCardsToBeDisclosed) {
      return 'All Table Cards to be Disclosed';
    }

    throw new Error('Unknown action');
  };

  return (
    <AnimatePresence>
      <motion.div
        className={clsx(styles.container, isInLoader && styles.inLoader)}
        variants={variants}
        initial="enter"
        animate="center"
        exit="exit"
        transition={{
          y: { type: 'spring', stiffness: 300, damping: 30 },
          opacity: { duration: 0.5 },
        }}>
        <p className={styles.title}>🔒 Verifying {getAction()} with Zero-Knowledge Proof</p>
        <p className={styles.description}>
          Your action is being cryptographically verified without revealing any private information — pure ZK magic.
        </p>
      </motion.div>
    </AnimatePresence>
  );
}

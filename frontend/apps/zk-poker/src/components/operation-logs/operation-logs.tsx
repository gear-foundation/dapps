import clsx from 'clsx';
import { motion } from 'framer-motion';
import { useState } from 'react';

import { useLogs } from '@/features/zk/hooks/use-logs';

import styles from './operation-logs.module.scss';

type Props = {
  isHidden?: boolean;
};

export function OperationLogs({ isHidden }: Props) {
  const { logs } = useLogs();
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className={clsx(styles.container, isHidden && styles.hidden)}>
      {isExpanded && !isHidden && (
        <motion.div className={styles.logsContainer}>
          {logs.map((log, index) => (
            <motion.div key={index} className={styles.logItem}>
              {log}
            </motion.div>
          ))}
        </motion.div>
      )}

      <motion.button className={styles.toggleButton} onClick={() => setIsExpanded(!isExpanded)}>
        {isExpanded ? 'Hide Logs' : 'Show Logs'}
      </motion.button>
    </div>
  );
}

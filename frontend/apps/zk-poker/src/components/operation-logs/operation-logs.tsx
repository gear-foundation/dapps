import { motion } from 'framer-motion';
import { useState } from 'react';

import { useLogs } from '@/features/zk/hooks/use-logs';

import styles from './operation-logs.module.scss';

export function OperationLogs() {
  const { logs } = useLogs();
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className={styles.container}>
      <motion.button className={styles.toggleButton} onClick={() => setIsExpanded(!isExpanded)}>
        Logs
      </motion.button>

      {isExpanded && (
        <motion.div className={styles.logsContainer}>
          {logs.map((log, index) => (
            <motion.div key={index} className={styles.logItem}>
              {log}
            </motion.div>
          ))}
        </motion.div>
      )}
    </div>
  );
}

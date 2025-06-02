import clsx from 'clsx';

import { CrossIcon } from '@/assets/images';

import { Button } from '../button';

import styles from './alert.module.scss';
import { AlertProps } from './alert.types';

export function Alert({ alert, close }: AlertProps) {
  const { content, options } = alert;
  const { type, title, style, isClosed } = options;

  return (
    <div className={styles.alert} style={style}>
      <div className={clsx(styles.header, styles[type])}>{title || type}</div>
      <div className={styles.body}>{content}</div>
      {isClosed && (
        <Button variant="text" className={styles.button} onClick={close}>
          <CrossIcon width={20} height={20} />
        </Button>
      )}
    </div>
  );
}
export { styles as alertStyles };

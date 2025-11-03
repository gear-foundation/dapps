import clsx from 'clsx';

import { Button } from '../Button';
import { Icon } from '../icon';

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
          <Icon name="cross" width={20} height={20} />
        </Button>
      )}
    </div>
  );
}

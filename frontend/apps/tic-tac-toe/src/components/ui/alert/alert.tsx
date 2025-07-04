import clsx from 'clsx';

import { Sprite } from '@/components/ui/sprite';

import { Button } from '../button';

import styles from './alert.module.scss';
import { AlertProps } from './alert.types';

function Alert({ alert, close }: AlertProps) {
  const { content, options } = alert;
  const { type, title, style, isClosed } = options;

  return (
    <div className={styles.alert} style={style}>
      <div className={clsx(styles.header, styles[type])}>{title || type}</div>
      <div className={styles.body}>{content}</div>
      {isClosed && (
        <Button variant="text" className={styles.button} onClick={close}>
          <Sprite name="close" size={20} />
        </Button>
      )}
    </div>
  );
}

export { Alert };

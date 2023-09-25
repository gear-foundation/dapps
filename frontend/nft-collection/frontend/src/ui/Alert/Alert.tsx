import { cx } from '@/utils';
import { AlertProps } from './Alert.interfaces';
import styles from './Alert.module.scss';
import { Button } from '../Button';
import CrossIcon from '@/assets/icons/cross-icon.svg';

function Alert({ alert, close }: AlertProps) {
  const { content, options } = alert;
  const { type, title, style, isClosed } = options;

  return (
    <div className={cx(styles.alert)} style={style}>
      <div className={cx(styles.header, styles[type])}>{title || type}</div>
      <div className={cx(styles.body)}>{content}</div>
      {isClosed && (
        <Button variant="icon" className={cx(styles.button)} onClick={close}>
          <CrossIcon />
        </Button>
      )}
    </div>
  );
}

export { styles as alertStyles, Alert };

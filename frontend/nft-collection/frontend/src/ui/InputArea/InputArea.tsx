import { cx } from '@/utils';
import styles from './InputArea.module.scss';
import { InputProps } from './InputArea.interfaces';

function InputArea({ size = 'medium', ...props }: InputProps) {
  return (
    <div className={cx(styles.wrapper, styles[`size-${size}`])}>
      <textarea {...props} className={cx(styles.input)} />
    </div>
  );
}

export { InputArea };

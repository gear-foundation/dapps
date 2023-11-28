import { ReactElement } from 'react';
import { cx } from 'utils';
import styles from './Warning.module.scss';
import { ReactComponent as ErrorIcon } from '../../assets/error-icon.svg';

type Props = {
  title: string;
  text: string;
  children?: ReactElement;
};

function Warning({ title, text, children }: Props) {
  return (
    <div className={cx(styles.container)}>
      <ErrorIcon />
      <div className={cx(styles.content)}>
        <span className={cx(styles.text, styles.textRed)}>{title}</span>
        <span className={cx(styles.text)}>{text}</span>
        {children}
      </div>
    </div>
  );
}

export { Warning };

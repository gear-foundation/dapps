import { Button as VaraButton, type ButtonProps as VaraButtonProps } from '@gear-js/vara-ui';
import clsx from 'clsx';

import styles from './button.module.scss';

export type ButtonProps = VaraButtonProps;

export function Button({ className, color = 'primary', ...props }: ButtonProps) {
  return (
    <div className={clsx(styles.wrapper, className)}>
      <VaraButton className={clsx(styles.button, styles[color])} {...props} />
    </div>
  );
}

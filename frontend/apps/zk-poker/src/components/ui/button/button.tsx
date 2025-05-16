import { Button as VaraButton, type ButtonProps as VaraButtonProps } from '@gear-js/vara-ui';
import clsx from 'clsx';

import styles from './button.module.scss';

export type ButtonProps = VaraButtonProps;

export function Button({ className, color = 'primary', size = 'default', ...props }: ButtonProps) {
  return (
    <div className={clsx(color !== 'transparent' && styles.wrapper, className)}>
      <VaraButton className={clsx(styles.button, styles[color], styles[size])} {...props} size={size} color={color} />
    </div>
  );
}

import { Button as VaraButton, type ButtonProps as VaraButtonProps } from '@gear-js/vara-ui';
import clsx from 'clsx';

import styles from './button.module.scss';

export type ButtonProps = VaraButtonProps & {
  rounded?: boolean;
};

export function Button({ className, color = 'primary', size = 'default', rounded, ...props }: ButtonProps) {
  return (
    <div className={clsx(color !== 'transparent' && !rounded && styles.wrapper, className)}>
      <VaraButton
        className={clsx(styles.button, styles[color], styles[size], rounded && styles.rounded)}
        {...props}
        size={size}
        color={color}
      />
    </div>
  );
}

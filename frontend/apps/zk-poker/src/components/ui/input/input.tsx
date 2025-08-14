import clsx from 'clsx';
import { forwardRef, InputHTMLAttributes } from 'react';

import styles from './input.module.scss';

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  className?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(({ className, ...props }, ref) => {
  return <input ref={ref} className={clsx(styles.input, className)} {...props} />;
});

Input.displayName = 'Input';

export { Input };

import { ButtonProps } from '@gear-js/vara-ui';

import styles from './Button.module.scss';

function Button({ type = 'button', text, form, onClick }: ButtonProps) {
  return (
    <button type={type} onClick={onClick} form={form} className={styles.button}>
      {text}
    </button>
  );
}

export { Button };

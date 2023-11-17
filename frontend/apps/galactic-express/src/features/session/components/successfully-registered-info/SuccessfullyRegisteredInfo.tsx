import { cx } from 'utils';
import styles from './SuccessfullyRegisteredInfo.module.scss';

function SuccessfullyRegisteredInfo() {
  return (
    <div className={cx(styles.container)}>
      <span className={cx(styles.text)}>You have successfully registered. </span>
      <span className={cx(styles.text)}>We are waiting for the administrator to start the game. </span>
    </div>
  );
}

export { SuccessfullyRegisteredInfo };

import battleshipImage from '@/assets/images/illustration-battleship.png';
import tailsGif from '@/assets/images/tails.gif';

import styles from './Illustration.module.scss';

export function Illustration() {
  return (
    <div className={styles.top}>
      <img src={battleshipImage} alt="battleship" width={300} className={styles.battleship} />
      <img src={tailsGif} alt="tails" width={157} className={styles.tails} />
    </div>
  );
}

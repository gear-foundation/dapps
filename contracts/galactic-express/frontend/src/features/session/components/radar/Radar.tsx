import earthSrc from '../../assets/earth.png';
import { LaunchState } from '../../types';
import styles from './Radar.module.scss';

type Props = {
  events: LaunchState['events'] | undefined;
};

function Radar({ events }: Props) {
  return (
    <div className={styles.container}>
      <img src={earthSrc} alt="" className={styles.image} />
    </div>
  );
}

export { Radar };

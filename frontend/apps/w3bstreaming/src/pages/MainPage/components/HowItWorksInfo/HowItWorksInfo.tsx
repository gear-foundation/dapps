import step1 from '@/assets/icons/about-card-1.png';
import step2 from '@/assets/icons/about-card-2.png';
import step3 from '@/assets/icons/about-card-3.png';
import step4 from '@/assets/icons/about-card-4.png';
import step5 from '@/assets/icons/about-card-5.png';
import { cx } from '@/utils';

import styles from './HowItWorksInfo.module.scss';

const steps = [
  [step1, step2, step3],
  [step4, step5],
];

function HowItWorksInfo() {
  return (
    <div className={cx(styles.content)}>
      <h3 className={cx(styles.title)}>How it works</h3>
      {steps.map((row, i) => (
        <div className={cx(styles.row)} key={`key ${row.join('')}`}>
          {row.map((item) => (
            <img className={cx(styles.step)} src={item} alt={`step ${i}`} key={item} />
          ))}
        </div>
      ))}
    </div>
  );
}

export { HowItWorksInfo };

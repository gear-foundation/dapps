import { useMemo } from 'react';
import clsx from 'clsx';
import styles from './Probability.module.scss';

type Props = {
  weather: number;
  payload: number;
  fuel: number;
};

function Probability({ weather, payload, fuel }: Props) {
  const probability = useMemo(() => {
    let result = (97.0 / 100.0) * ((95.0 - 2.0 * weather) / 100.0) * ((90.0 - 2.0 * weather) / 100.0);

    if (payload >= 80) result = (97.0 / 100.0) * ((85.0 - 2.0 * weather) / 100.0) * ((90.0 - 2.0 * weather) / 100.0);

    if (fuel >= 80) result = (87.0 / 100.0) * ((95.0 - 2.0 * weather) / 100.0) * ((90.0 - 2.0 * weather) / 100.0);

    if (fuel >= 80 && payload >= 80)
      result = (87.0 / 100.0) * ((85.0 - 2.0 * weather) / 100.0) * ((90.0 - 2.0 * weather) / 100.0);

    return Math.round(result * 100);
  }, [weather, payload, fuel]);

  const probabilityId = useMemo(() => {
    if (probability <= 35) return 'low';
    if (probability <= 70) return 'medium';

    return 'high';
  }, [probability]);

  const valueClassName = clsx(styles.value, styles[probabilityId]);

  return (
    <p className={styles.probability}>
      Success Probability:
      <span className={valueClassName}>{probability}%</span>
    </p>
  );
}

export { Probability };

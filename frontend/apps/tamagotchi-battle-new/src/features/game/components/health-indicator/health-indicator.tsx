import clsx from 'clsx';
import styles from './health-indicator.module.scss';

type HealthIndicatorProps = {
  currentHealth: number;
  maxHealth?: number;
  prevHealth?: number;
  size?: 'md' | 'sm';
};

const HealthIndicator = ({ currentHealth, maxHealth = 100, prevHealth, size = 'md' }: HealthIndicatorProps) => {
  const segments = 10;
  const healthPerSegment = maxHealth / segments;

  const healthSegments = Array.from({ length: segments }, (_, i) => {
    const segmentHealthStart = i * healthPerSegment;
    const segmentHealthEnd = (i + 1) * healthPerSegment;

    const isCurrentPartiallyFilled = currentHealth < segmentHealthEnd && currentHealth > segmentHealthStart;
    const isPrevPartiallyFilled = prevHealth && prevHealth < segmentHealthEnd && prevHealth > segmentHealthStart;

    const currentFillPercent = isCurrentPartiallyFilled
      ? ((currentHealth - segmentHealthStart) / healthPerSegment) * 100
      : currentHealth >= segmentHealthEnd
      ? 100
      : 0;

    const damageFillPercent = isPrevPartiallyFilled
      ? ((prevHealth - segmentHealthStart) / healthPerSegment) * 100 - currentFillPercent
      : prevHealth && prevHealth >= segmentHealthEnd && currentHealth < segmentHealthEnd
      ? 100 - currentFillPercent
      : 0;

    return (
      <div key={i} className={clsx(styles.segment, size === 'sm' && styles.small)}>
        <div className={styles.inner}>
          <div className={styles.health} style={{ width: `${currentFillPercent}%` }} />

          {damageFillPercent > 0 && (
            <div
              className={styles.damage}
              style={{
                left: `${currentFillPercent}%`,
                width: `${damageFillPercent}%`,
              }}
            />
          )}
        </div>
        <div className={styles.shadow} style={{ width: `${currentFillPercent}%` }}></div>
      </div>
    );
  });

  return <div className={styles.wrapper}>{healthSegments}</div>;
};

export { HealthIndicator };

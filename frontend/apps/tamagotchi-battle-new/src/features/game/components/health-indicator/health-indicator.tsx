import styles from './health-indicator.module.scss';

type HealthIndicatorProps = {
  currentHealth: number;
  maxHealth?: number;
  prevHealth?: number;
};

const HealthIndicator = ({ currentHealth, maxHealth = 100, prevHealth }: HealthIndicatorProps) => {
  const segments = 10;
  const healthPerSegment = maxHealth / segments;

  // Создаем массив для сегментов здоровья
  const healthSegments = Array.from({ length: segments }, (_, i) => {
    const segmentHealthStart = i * healthPerSegment;
    const segmentHealthEnd = (i + 1) * healthPerSegment;

    const isCurrentPartiallyFilled = currentHealth < segmentHealthEnd && currentHealth > segmentHealthStart;
    const isPrevPartiallyFilled = prevHealth && prevHealth < segmentHealthEnd && prevHealth > segmentHealthStart;

    // Процент заполнения текущего здоровья в сегменте
    const currentFillPercent = isCurrentPartiallyFilled
      ? ((currentHealth - segmentHealthStart) / healthPerSegment) * 100
      : currentHealth >= segmentHealthEnd
      ? 100
      : 0;

    // Процент урона в сегменте
    const damageFillPercent = isPrevPartiallyFilled
      ? ((prevHealth - segmentHealthStart) / healthPerSegment) * 100 - currentFillPercent
      : prevHealth && prevHealth >= segmentHealthEnd && currentHealth < segmentHealthEnd
      ? 100 - currentFillPercent
      : 0;

    return (
      <div
        key={i}
        style={{
          position: 'relative',
          height: '16px',
          width: '13.5px',
          borderRadius: '4px',
          margin: '0 3px',
        }}>
        <div
          style={{
            position: 'relative',
            height: '100%',
            width: '100%',
            borderRadius: '4px',
            backgroundColor: '#0B0B0B',
            overflow: 'hidden',
          }}>
          {/* Зелёный цвет для текущего здоровья */}
          <div
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              height: '100%',
              width: `${currentFillPercent}%`,
              backgroundColor: '#00FFC4',
              boxShadow: '0px 0.6px 1.8px 0px #00000033 inset',
            }}
          />
          {/* Красный цвет для урона */}
          {damageFillPercent > 0 && (
            <div
              style={{
                position: 'absolute',
                top: 0,
                left: `${currentFillPercent}%`,
                height: '100%',
                width: `${damageFillPercent}%`,
                backgroundColor: '#CA445F',
              }}
            />
          )}
        </div>
        <div className={styles.shadow} style={{ width: `${currentFillPercent}%` }}></div>
      </div>
    );
  });

  return <div style={{ display: 'flex', alignItems: 'center' }}>{healthSegments}</div>;
};

export { HealthIndicator };

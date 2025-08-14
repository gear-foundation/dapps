import styles from './stats.module.scss';

type Stat = {
  value: number | string;
  label: string;
};

type Props = {
  items: Stat[];
};

const Stats = ({ items }: Props) => {
  return (
    <div className={styles.stats}>
      {items.map((stat) => (
        <div className={styles.stat} key={stat.label}>
          <span className={styles.statLabel}>{stat.label}</span>
          <span className={styles.statValue}>{stat.value}</span>
        </div>
      ))}
    </div>
  );
};

export { Stats };

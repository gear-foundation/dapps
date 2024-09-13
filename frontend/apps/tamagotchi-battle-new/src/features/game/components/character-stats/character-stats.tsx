import { Text } from '@/components';
import styles from './character-stats.module.scss';
import { AttackIcon, DefenseIcon, DodgeIcon, HealthIcon } from '../../assets/images';

type CharacterStats = {
  icon: React.ReactNode;
  title: string;
  value: number;
};

const mockStats: CharacterStats[] = [
  { icon: <HealthIcon />, title: 'Health', value: 20 },
  { icon: <AttackIcon />, title: 'Attack', value: 10 },
  { icon: <DefenseIcon />, title: 'Defense', value: 10 },
  { icon: <DodgeIcon />, title: 'Dodge', value: 10 },
];

export const CharacterStats = () => {
  const drawRow = ({ icon, title, value }: CharacterStats) => (
    <div className={styles.row}>
      {icon}
      <Text size="sm" className={styles.text}>
        {title}
      </Text>

      {value}
    </div>
  );

  return <div className={styles.container}>{mockStats.map((stats) => drawRow(stats))}</div>;
};

import { useEffect } from 'react';
import { Button } from '@gear-js/vara-ui';
import { useForm } from '@mantine/form';
import { Text } from '@/components';
import { Heading } from '@/components/ui/heading';
import { AttackIcon, CaretRightIcon, DefenceIcon, DodgeIcon } from '../../assets/images';
import { CharacterStatsFormValues } from '../../types';
import styles from './character-stats-form.module.scss';

type Stats = 'attack' | 'defence' | 'dodge';

type CharacterStats = {
  icon: React.ReactNode;
  name: Stats;
  description: string;
  maxCount: number;
  minCount: number;
  percentPerPoint?: number;
};

const charStats: CharacterStats[] = [
  {
    icon: <AttackIcon />,
    name: 'attack',
    description: 'The strength of the damage you do to the opponent.',
    maxCount: 20,
    minCount: 10,
  },
  {
    icon: <DefenceIcon />,
    name: 'defence',
    description: "Reflects the opponent's attack back to them. Each point equals 10%.",
    percentPerPoint: 10,
    maxCount: 10,
    minCount: 0,
  },
  {
    icon: <DodgeIcon />,
    name: 'dodge',
    description: 'The chance to fully evade opponent’s attack. Each point increases chance by 4%.',
    percentPerPoint: 4,
    maxCount: 10,
    minCount: 0,
  },
];

type CharacterStatsFormProps = {
  onValuesChange?: (values: CharacterStatsFormValues, isValid: boolean) => void;
};

export const CharacterStatsForm = ({ onValuesChange }: CharacterStatsFormProps) => {
  const statsForm = useForm({
    initialValues: {
      attack: 10,
      defence: 0,
      dodge: 0,
    },
    transformValues: (values) => {
      return {
        attack: Math.min(values.attack, charStats[0].maxCount),
        defence: Math.min(values.defence, charStats[1].maxCount),
        dodge: Math.min(values.dodge, charStats[2].maxCount),
      };
    },
  });

  const { getInputProps, setFieldValue, values } = statsForm;
  const initialPoints = 10;
  const availablePoints = 20 + initialPoints - values.attack - values.defence - values.dodge;

  useEffect(() => {
    const isValid = availablePoints === 0;
    onValuesChange?.(values, isValid);
  }, [values, availablePoints, onValuesChange]);

  const drawRow = ({ icon, name, percentPerPoint, maxCount, minCount, description }: CharacterStats) => {
    const getValidCount = (count: number) => {
      return Math.max(minCount, Math.min(Number(count), maxCount));
    };

    const value = values[name];

    return (
      <div key={name}>
        <div className={styles.row}>
          {icon}
          <Text size="md" className={styles.text}>
            {name} {percentPerPoint && <span>({percentPerPoint * value}%)</span>}:
          </Text>

          <Button
            color="transparent"
            size="small"
            icon={CaretRightIcon}
            className={styles.arrowLeft}
            disabled={values[name] <= minCount}
            onClick={() => setFieldValue(name, value - 1)}
          />
          <div className={styles.input}>
            <input
              {...getInputProps(name)}
              inputMode="numeric"
              type="number"
              max={maxCount}
              min={minCount}
              onBlur={(event) => {
                setFieldValue(name, getValidCount(Number(event.currentTarget.value)));
              }}
            />
            <div className={styles.border} />
          </div>
          <Button
            color="transparent"
            size="small"
            icon={CaretRightIcon}
            onClick={() => setFieldValue(name, value + 1)}
            disabled={values[name] >= maxCount || availablePoints === 0}
          />

          <Button
            color="transparent"
            className={styles.max}
            onClick={() => setFieldValue(name, Math.min(maxCount, value + availablePoints))}>
            <Text size="xs" weight="medium">
              {maxCount} max
            </Text>
          </Button>
        </div>
        <Text size="xs" className={styles.description}>
          {description}
        </Text>
      </div>
    );
  };

  return (
    <div className={styles.container}>
      <Heading size="xs" weight="bold" className={styles.title}>
        Set Character's Attributes
      </Heading>

      <Text size="sm">
        <span className={styles.points}>{availablePoints} points</span> are available to distribute.
      </Text>

      <form className={styles.stats}>{charStats.map((stats) => drawRow(stats))}</form>
    </div>
  );
};

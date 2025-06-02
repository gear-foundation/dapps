import { Text } from '@/components';

import { LoaderIcon } from '../../assets/images';

import styles from './game-spinner.module.scss';

type GameSpinnerProps = {
  text: string;
};

export const GameSpinner = ({ text }: GameSpinnerProps) => {
  return (
    <div className={styles.wrapper}>
      <LoaderIcon />
      <Text size="xs" weight="semibold" className={styles.text}>
        {text}
      </Text>
    </div>
  );
};

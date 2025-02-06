import ArrowRight from '@/assets/images/icons/arrow-right.svg?react';
import AndroidLine from '@/assets/images/icons/android-line.svg?react';
import AdminLine from '@/assets/images/icons/admin-line.svg?react';
import SearchLine from '@/assets/images/icons/search-line.svg?react';
import { Button } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import styles from './SelectGameMode.module.scss';
import { useGameMode } from '../../hooks';
import { Illustration } from '../illustration';

export default function SelectGameMode() {
  const { setGameMode } = useGameMode();

  const gameMods = [
    {
      icon: <AndroidLine className={styles.buttonOptionSvg} />,
      heading: `Play with an on-chain program`,
      description: `Start a single player game with a program on Vara.`,
      onClick: () => setGameMode('single'),
    },
    {
      icon: <SearchLine className={styles.buttonOptionSvg} />,
      heading: `Join a peer-to-peer game`,
      description: `Enter the game's address to join the game.`,
      onClick: () => setGameMode('find'),
    },
    {
      icon: <AdminLine className={styles.buttonOptionSvg} />,
      heading: `Create a peer-to-peer game`,
      description: `Create a new game and invite a friend to play.`,
      onClick: () => setGameMode('create'),
    },
  ];

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <Illustration />
        <div className={styles.header}>
          <Heading className={styles.mainHeading}>Select game mode</Heading>
        </div>
        <div className={styles.controlsWrapper}>
          {gameMods.map(({ icon, heading, description, onClick }) => (
            <Button key={heading} variant="outline" className={styles.buttonOption} onClick={onClick}>
              <span className={styles.buttonOptionContent}>
                <span className={styles.buttonOptionHeading}>
                  {icon}
                  {heading}
                </span>
                <span className={styles.buttonOptionDescription}>{description}</span>
              </span>
              <ArrowRight className={styles.buttonOptionSvg} />
            </Button>
          ))}
        </div>
      </div>
    </div>
  );
}

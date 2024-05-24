import { useNavigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import battleshipImage from '@/assets/images/illustration-battleship.png';
import { ReactComponent as ArrowRight } from '@/assets/images/icons/arrow-right.svg';
import { ReactComponent as AndroidLine } from '@/assets/images/icons/android-line.svg';
import { ReactComponent as AdminLine } from '@/assets/images/icons/admin-line.svg';
import { ReactComponent as SearchLine } from '@/assets/images/icons/search-line.svg';
import { Button } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import styles from './SelectGameMode.module.scss';
import { useGameMode } from '../../hooks';

export default function SelectGameMode() {
  const { setGameMode } = useGameMode();

  const gameMods = [
    {
      icon: <AndroidLine className={styles.buttonOptionSvg} />,
      heading: `Play with a smart contract`,
      description: `Start the game without any preparations right now.`,
      onClick: () => setGameMode('single'),
    },
    {
      icon: <SearchLine className={styles.buttonOptionSvg} />,
      heading: `Find a private game`,
      description: `To find the game, you need to enter the administrator's address.`,
      onClick: () => setGameMode('find'),
    },
    {
      icon: <AdminLine className={styles.buttonOptionSvg} />,
      heading: `Create a game in admin mode`,
      description: `Create a game and specify your participation rules.`,
      onClick: () => setGameMode('create'),
    },
  ];

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.top}>
          <img src={battleshipImage} alt="battleship" width={300} />
        </div>
        <div className={styles.header}>
          <Heading className={styles.mainHeading}>Select game mode</Heading>
          <div>
            <Text className={styles.mainText}>Which mode shall we play in today?</Text>
          </div>
        </div>
        <div className={styles.controlsWrapper}>
          {gameMods.map(({ icon, heading, description, onClick }) => (
            <Button variant="outline" className={styles.buttonOption} onClick={onClick}>
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

import { useSetAtom } from 'jotai';
import { useNavigate } from 'react-router-dom';
import { Card } from '@/components';
import { CardButton } from '@/components/ui/card-button';
import styles from './select-game-mode.module.scss';
import { gameStatusAtom } from '../../store';
import { CodeSlashIcon, MagicLineIcon } from '../../assets/images';
import { Background } from '../background';

export const SelectGameMode = () => {
  const navigate = useNavigate();
  const setGameStatus = useSetAtom(gameStatusAtom);

  return (
    <Background>
      <Card title="Tamagotchi Battle" subTitle="Select game mode" className={styles.card}>
        <div className={styles.container}>
          <CardButton
            onClick={() => setGameStatus('import')}
            icon={<CodeSlashIcon />}
            title="Import Character from Program"
            subTitle="Enter the program ID to view your character."
          />
          <CardButton
            onClick={() => setGameStatus('generate')}
            icon={<MagicLineIcon />}
            title="Generate Character Without a Code"
            subTitle="Simply generate a random appearance and attributes."
          />
        </div>
      </Card>
    </Background>
  );
};

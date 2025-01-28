import { EzTransactionsSwitch } from 'gear-ez-transactions';
import { useNavigate } from 'react-router-dom';

import { Card } from '@/components';
import { ALLOWED_SIGNLESS_ACTIONS, ROUTES } from '@/app/consts';
import { Background } from '@/features/game/components';
import { CardButton } from '@/components/ui/card-button';
import { CodeSlashIcon, MagicLineIcon } from '@/features/game/assets/images';
import { useResetCharacterStats } from '@/features/game/hooks';

import styles from './home.module.scss';

export default function Home() {
  const navigate = useNavigate();
  useResetCharacterStats();

  return (
    <Background>
      <Card title="Web3 Warriors Battle" description="Select game mode" className={styles.card} size="lg">
        <div className={styles.container}>
          <CardButton
            onClick={() => navigate(ROUTES.IMPORT_CHARACTER)}
            icon={<CodeSlashIcon />}
            title="Import Character from Program"
            description="Enter the program ID to view your character."
          />

          <CardButton
            onClick={() => navigate(ROUTES.GENERATE_CHARACTER)}
            icon={<MagicLineIcon />}
            title="Generate Character Without a Code"
            description="Simply generate a random appearance and attributes."
          />
        </div>

        <EzTransactionsSwitch allowedActions={ALLOWED_SIGNLESS_ACTIONS} />
      </Card>
    </Background>
  );
}

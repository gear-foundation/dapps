import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from 'gear-ez-transactions';
import { useNavigate } from 'react-router-dom';

import { Wallet } from '@dapps-frontend/ui';

import { ALLOWED_SIGNLESS_ACTIONS, ROUTES } from '@/app/consts';
import { Card } from '@/components';
import { CardButton } from '@/components/ui/card-button';
import { CodeSlashIcon, MagicLineIcon } from '@/features/game/assets/images';
import { Background } from '@/features/game/components';
import { useResetCharacterStats } from '@/features/game/hooks';

import styles from './home.module.scss';

export function Home() {
  const { account } = useAccount();
  const navigate = useNavigate();
  useResetCharacterStats();

  if (!account)
    return (
      <Background>
        <Card
          size="lg"
          title="Web3 Warriors Battle"
          description="Create your Warrior character and engage in battles with other players."
          className={styles.authCard}>
          <Wallet />
        </Card>
      </Background>
    );

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

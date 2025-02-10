import { Wallet } from '@dapps-frontend/ui';

import { Card } from '@/components/ui/card';
import { Background } from '@/features/game/components';

import styles from './not-authorized.module.scss';

export function NotAuthorized() {
  return (
    <Background>
      <Card
        size="lg"
        title="Web3 Warriors Battle"
        description="Create your Warrior character and engage in battles with other players."
        className={styles.card}>
        <Wallet theme="vara" />
      </Card>
    </Background>
  );
}

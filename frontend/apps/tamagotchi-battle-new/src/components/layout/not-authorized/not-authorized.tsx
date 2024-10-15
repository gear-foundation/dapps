import { Background } from '@/features/game/components';
import { Card } from '@/components/ui/card';
import { Wallet } from '@dapps-frontend/ui';
import styles from './not-authorized.module.scss';

export function NotAuthorized() {
  return (
    <Background>
      <Card
        size="lg"
        title="Tamagotchi Battle"
        description="Create your Tamagotchi character and engage in battles with other players."
        className={styles.card}>
        <Wallet theme="vara" />
      </Card>
    </Background>
  );
}

import styles from './not-authorized.module.scss';
import { Link } from 'react-router-dom';
import { buttonVariants } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';

const testnetURL = import.meta.env.VITE_TESTNET_WEBSITE_ADDRESS;

export function NotAuthorized() {
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.header}>
          <Heading>
            <TextGradient>Tic-Tac-Toe</TextGradient>
          </Heading>
          <div>
            <Text size="lg">You are currently not part of the Vara Network Testnet.</Text>
            <Text size="lg">Please register using the referral link in the Testnet portal:</Text>
          </div>
        </div>
        <Link to={testnetURL} target="_blank" className={buttonVariants()}>
          Vara Network Testnet
        </Link>
        <div className={styles.bottom}>
          <Text className={styles.muted} size="lg">
            More information can be found in our{' '}
            <Link to="https://discord.gg/x8ZeSy6S6K" target="_blank" className={styles.link}>
              Discord
            </Link>{' '}
            and{' '}
            <Link to="https://t.me/VaraNetwork_Global" target="_blank" className={styles.link}>
              Telegram
            </Link>
          </Text>
        </div>
      </div>
    </div>
  );
}

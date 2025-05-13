import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { AnimatePresence } from 'framer-motion';
import { EzTransactionsSwitch } from 'gear-ez-transactions';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';
import { WalletConnect } from '@/features/wallet';

import styles from './login.module.scss';

export default function Login() {
  const navigate = useNavigate();
  const { account } = useAccount();

  const [isOpen, setIsOpen] = useState(false);
  const openWallet = () => setIsOpen(true);
  const closeWallet = () => setIsOpen(false);

  const onClickStartGame = () => {
    if (!account) throw new Error('Account is not found');
    navigate('/game');
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.header}>
          <Heading size="md" className={styles.mainHeading}>
            <TextGradient>Battleship Game</TextGradient>
          </Heading>
          <div>
            <Text size="md" className={styles.mainText}>
              Welcome to the ZK Poker game, where you can compete with an on-chain program.
              {!account && ' To start the game, connect your wallet.'}
            </Text>
          </div>
        </div>
        <div className={styles.controlsWrapper}>
          <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />

          <Button className={styles.startMenuButton} onClick={account ? onClickStartGame : openWallet}>
            {account ? 'Start the Game' : 'Connect wallet'}
          </Button>
          {account && (
            <Button className={styles.startMenuButton} color="grey">
              Back
            </Button>
          )}
        </div>
      </div>

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}

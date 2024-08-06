import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';
import { WalletConnect } from '@/features/wallet';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import { Illustration } from '@/features/game/components';
import { useGameMode } from '@/features/game/hooks';
import styles from './login.module.scss';

export default function Login() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { resetGameMode } = useGameMode();

  const [isOpen, setIsOpen] = useState(false);
  const openWallet = () => setIsOpen(true);
  const closeWallet = () => setIsOpen(false);

  const onClickStartGame = () => {
    if (!account) throw new Error('Account is not found');
    navigate('/game');
  };

  const onClickBack = () => {
    resetGameMode();
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <Illustration />
        <div className={styles.header}>
          <Heading size="md" className={styles.mainHeading}>
            <TextGradient>Battleship Game</TextGradient>
          </Heading>
          <div>
            <Text size="md" className={styles.mainText}>
              Welcome to the 'Battleship' game, where you can compete with an on-chain program.
              {!account && ' To start the game, connect your wallet.'}
            </Text>
          </div>
        </div>
        <div className={styles.controlsWrapper}>
          <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />

          <Button className={styles.startGameButton} onClick={account ? onClickStartGame : openWallet}>
            {account ? 'Start the Game' : 'Connect wallet'}
          </Button>
          {account && (
            <Button className={styles.startGameButton} color="grey" onClick={onClickBack}>
              Back
            </Button>
          )}
        </div>
      </div>

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}

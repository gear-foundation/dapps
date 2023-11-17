import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import { useAccount } from '@gear-js/react-hooks';

import { Button, buttonVariants } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';
import battleshipImage from '@/assets/images/illustration-battleship.png';
import { WalletConnect } from '@/features/wallet';

import styles from './login.module.scss';

export default function Login() {
  const navigation = useNavigate();
  const { account } = useAccount();

  const [isOpen, setIsOpen] = useState(false);

  const openWallet = () => setIsOpen(true);
  const closeWallet = () => setIsOpen(false);

  const onClickStartGame = () => {
    navigation('/game');
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.header}>
          <Heading>
            <TextGradient>Battleship game</TextGradient>
          </Heading>
          <div>
            <Text size="lg">
              Welcome to the classic Battleship game, where you will compete against a smart contract.
              {!account && ' To start the game, connect your wallet.'}
            </Text>
          </div>
        </div>
        <Button className={buttonVariants()} onClick={account ? onClickStartGame : openWallet}>
          {account ? 'Start the Game' : 'Connect wallet'}
        </Button>
        <div className={styles.bottom}>
          <img src={battleshipImage} alt="" width={300} />
        </div>
      </div>

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}

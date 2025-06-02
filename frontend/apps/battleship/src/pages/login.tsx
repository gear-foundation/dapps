import { useAccount, useAlert } from '@gear-js/react-hooks';
import { AnimatePresence } from 'framer-motion';
import { useGaslessTransactions, EzTransactionsSwitch, useSignlessTransactions } from 'gear-ez-transactions';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';
import battleshipImage from '@/assets/images/illustration-battleship.png';
import { Button, buttonVariants } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';
import { WalletConnect } from '@/features/wallet';

import styles from './login.module.scss';

export default function Login() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const alert = useAlert();

  const signless = useSignlessTransactions();
  const gasless = useGaslessTransactions();

  const [isOpen, setIsOpen] = useState(false);
  const openWallet = () => setIsOpen(true);
  const closeWallet = () => setIsOpen(false);

  const onClickStartGame = () => {
    if (!account) throw new Error('Account is not found');
    // withVoucherRequest? to handle condition inside of gasless context
    if (!gasless.isEnabled || gasless.voucherId || signless.isActive) return navigate('/game');

    gasless
      .requestVoucher(account.address)
      .then(() => navigate('/game'))
      .catch(({ message }: Error) => alert.error(message));
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.top}>
          <img src={battleshipImage} alt="battleship" width={300} />
        </div>
        <div className={styles.header}>
          <Heading size="md" className={styles.mainHeading}>
            <TextGradient>Battleship Game</TextGradient>
          </Heading>
          <div>
            <Text size="lg" className={styles.mainText}>
              Welcome to the classic Battleship game, where you will compete against a smart contract. To start the
              game, connect your wallet.
              {!account && ' To start the game, connect your wallet.'}
            </Text>
          </div>
        </div>
        <div className={styles.controlsWrapper}>
          <Button
            className={(buttonVariants(), styles.startGameButton)}
            onClick={account ? onClickStartGame : openWallet}>
            {account ? 'Start the Game' : 'Connect wallet'}
          </Button>

          <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
        </div>
      </div>

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}

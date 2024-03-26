import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import battleshipImage from '@/assets/images/illustration-battleship.png';
import { Button, buttonVariants } from '@/components/ui/button/button';
import { Heading } from '@/components/ui/heading';
import { Text } from '@/components/ui/text';
import { TextGradient } from '@/components/ui/text-gradient';
import { WalletConnect } from '@/features/wallet';
import styles from './login.module.scss';
import { useGaslessTransactions, EzTransactionsSwitch, useSignlessTransactions } from '@dapps-frontend/ez-transactions';

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
        <div className={styles.controlsWrapper}>
          <Button
            className={(buttonVariants(), styles.startGameButton)}
            onClick={account ? onClickStartGame : openWallet}>
            {account ? 'Start the Game' : 'Connect wallet'}
          </Button>

          <EzTransactionsSwitch />
        </div>

        <div className={styles.bottom}>
          <img src={battleshipImage} alt="" width={300} />
        </div>
      </div>

      <AnimatePresence>{isOpen && <WalletConnect onClose={closeWallet} />}</AnimatePresence>
    </div>
  );
}

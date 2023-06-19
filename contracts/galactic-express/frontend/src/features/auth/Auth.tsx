import { useState } from 'react';
import { Button, Modal } from '@gear-js/ui';
import { ReactComponent as ExitSVG } from 'assets/images/icons/exit.svg';
import { GaslessAccount, GaslessAccountModal } from '../gasless-account';
import { Wallet, WalletModal } from '../wallet';
import { useAuth } from './Context';
import styles from './Auth.module.scss';

type Props = {
  hideResetButton?: boolean;
};

function Auth({ hideResetButton }: Props) {
  const { authType, setAuthType } = useAuth();

  const [isTypeModalOpen, setIsTypeModalOpen] = useState(false);
  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);
  const [isGasslessAccountModalOpen, setIsGasslessAccountModalOpen] = useState(false);

  const openTypeModal = () => setIsTypeModalOpen(true);
  const closeTypeModal = () => setIsTypeModalOpen(false);

  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  const openGaslessAccountModal = () => setIsGasslessAccountModalOpen(true);
  const closeGaslessAccountModal = () => setIsGasslessAccountModalOpen(false);

  const handleWalletButtonClick = () => {
    setAuthType('wallet');
    openWalletModal();
    closeTypeModal();
  };

  const handleGaslessButtonClick = () => {
    setAuthType('gasless');
    openGaslessAccountModal();
    closeTypeModal();
  };

  const handleResetButtonClick = () => {
    setAuthType('');
  };

  return (
    <>
      <div className={styles.auth}>
        {authType === 'wallet' && <Wallet />}
        {authType === 'gasless' && <GaslessAccount />}

        {authType ? (
          !hideResetButton && (
            <Button
              icon={ExitSVG}
              color="transparent"
              className={styles.resetButton}
              onClick={handleResetButtonClick}
            />
          )
        ) : (
          <Button text="Auth" onClick={openTypeModal} />
        )}
      </div>

      {isTypeModalOpen && (
        <Modal heading="Select auth type" close={closeTypeModal} className={styles.modal}>
          <Button text="Extension Wallet" color="lightGreen" size="small" onClick={handleWalletButtonClick} />
          <Button text="Gasless Service" color="lightGreen" size="small" onClick={handleGaslessButtonClick} />
        </Modal>
      )}

      {isWalletModalOpen && <WalletModal onClose={closeWalletModal} />}
      {isGasslessAccountModalOpen && <GaslessAccountModal onClose={closeGaslessAccountModal} />}
    </>
  );
}

export { Auth };

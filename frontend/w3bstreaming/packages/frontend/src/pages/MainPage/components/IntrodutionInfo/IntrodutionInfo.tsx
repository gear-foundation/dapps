import { useState } from 'react';
import { Button } from '@ui';
import styles from './IntrodutionInfo.module.scss';
import mainFrame from '@/assets/icons/main-page-frame.png';
import animImg from '@/assets/icons/main-page-wara-anim.png';
import courtain from '@/assets/icons/courtain.png';
import { cx } from '@/utils';
import { WalletModal } from '@/features/Wallet/components';

function IntrodutionInfo() {
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleOpenWalletModal = () => {
    setIsWalletModalOpen(true);
  };

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  return (
    <div className={cx(styles.content)}>
      <div className={cx(styles.left)}>
        <div>
          <h1 className={cx(styles['main-title'])}>Vara </h1>
          <h1 className={cx(styles['main-title'], styles['main-title-with-gradient'])}>P2P Streaming</h1>
        </div>
        <p className={cx(styles['main-description'])}>
          Revolutionize media consumption with seamless audio and video streaming using P2P technology. Stream and share
          content directly from your devices, eliminating the need for centralized servers
        </p>
        <Button label="Connect Wallet" variant="primary" size="large" onClick={handleOpenWalletModal} />
      </div>
      <div className={cx(styles.right)}>
        <img src={mainFrame} alt="united people" className={cx(styles['lower-img'])} />
        <img src={courtain} alt="courtain" className={cx(styles.courtain)} />
        <img src={animImg} alt="some stuff" className={cx(styles['upper-img'])} />
      </div>
      {isWalletModalOpen && <WalletModal onClose={handleCloseWalletModal} />}
    </div>
  );
}

export { IntrodutionInfo };

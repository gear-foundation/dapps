import { Wallet } from 'features/wallet';
import { NodeSwitch } from 'features/node-switch';
import { ContractAddress, useContractAddress } from 'features/contract-address';
import { Logo } from './logo';
import styles from './Header.module.scss';

function Header() {
  const contractAddress = useContractAddress();

  return (
    <header className={styles.header}>
      <Logo />

      <div className={styles.wrapper}>
        <div className={styles.addresses}>
          <ContractAddress />
          {contractAddress && <span className={styles.separator} />}
          <NodeSwitch />
        </div>

        <Wallet />
      </div>
    </header>
  );
}

export { Header };

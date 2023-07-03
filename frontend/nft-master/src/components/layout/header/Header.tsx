import { Wallet } from 'features/wallet';
import { NodeSwitch } from 'features/node-switch';
import { ContractAddress, useContractAddress } from 'features/contract-address';
import { Logo } from './logo';
import styles from './Header.module.scss';
import { Container } from '../container';

function Header() {
  const contractAddress = useContractAddress();

  return (
    <header>
      <Container className={styles.container}>
        <Logo />

        <div className={styles.wrapper}>
          <div className={styles.addresses}>
            <ContractAddress />
            {contractAddress && <span className={styles.separator} />}
            <NodeSwitch />
          </div>

          <Wallet />
        </div>
      </Container>
    </header>
  );
}

export { Header };

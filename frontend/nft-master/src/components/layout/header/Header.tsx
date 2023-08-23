import { useState } from 'react';
import { Wallet } from 'features/wallet';
import { NodeSwitch } from 'features/node-switch';
import { ContractAddress, useContractAddress } from 'features/contract-address';
import { Search } from 'features/nfts';
import { useResizeEffect } from 'hooks';
import { Button } from 'components';
import { CrossIcon, HamburgerIcon } from 'assets/images';
import clsx from 'clsx';
import { Container } from '../container';
import { Logo } from './logo';
import styles from './Header.module.scss';

function Header() {
  const { contractAddress } = useContractAddress();

  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const toggleMenu = () => setIsMenuOpen((prevValue) => !prevValue);
  const closeMenu = () => setIsMenuOpen(false);

  useResizeEffect(closeMenu);

  return (
    <header>
      <Container className={styles.container}>
        <Logo />

        <div className={styles.mobileMenuWrapper}>
          <Button variant="white" className={styles.button} onClick={toggleMenu}>
            {isMenuOpen ? <CrossIcon /> : <HamburgerIcon />}
          </Button>

          {isMenuOpen && (
            <ul className={styles.list}>
              <li className={styles.item}>
                <ContractAddress />
              </li>
              <li className={styles.item}>
                <NodeSwitch />
              </li>
              <li className={clsx(styles.item, styles['item--wallet'])}>
                <Wallet />
              </li>
            </ul>
          )}
        </div>

        <div className={styles.configuration}>
          <Search />

          <div className={styles.desktopMenu}>
            <div className={styles.addresses}>
              <ContractAddress />
              {contractAddress && <span className={styles.separator} />}
              <NodeSwitch />
            </div>

            <Wallet className={styles.desktopWallet} />
          </div>
        </div>
      </Container>
    </header>
  );
}

export { Header };

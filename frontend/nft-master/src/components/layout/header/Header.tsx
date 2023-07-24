import { useState } from 'react';
import { Button } from '@gear-js/ui';
import { ReactComponent as HamburgerSVG } from 'assets/images/icons/hamburger.svg';
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross.svg';
import { Wallet } from 'features/wallet';
import { NodeSwitch } from 'features/node-switch';
import { ContractAddress, useContractAddress } from 'features/contract-address';
import { Search } from 'features/nfts';
import { useResizeEffect } from 'hooks';
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
          <Button
            icon={isMenuOpen ? CrossSVG : HamburgerSVG}
            color="transparent"
            className={styles.button}
            onClick={toggleMenu}
          />

          {isMenuOpen && (
            <ul className={styles.list}>
              <li className={styles.item}>
                <ContractAddress />
              </li>
              <li className={styles.item}>
                <NodeSwitch />
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

            <Wallet />
          </div>
        </div>
      </Container>
    </header>
  );
}

export { Header };

import { useAccount } from '@gear-js/react-hooks';
import { useNavigate } from 'react-router-dom';

// import { WalletInfo } from '@/features/Wallet/components';
import { routes } from '@/App.routes';
import closeMenuIcon from '@/assets/icons/cross-icon.svg';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { BurgerMenuProps } from './BurgerMenu.interfaces';
import styles from './BurgerMenu.module.scss';

function BurgerMenu({ burgerMenuHandler }: BurgerMenuProps) {
  const navigate = useNavigate();
  const { account } = useAccount();
  const mobileMenuItems = Object.keys(routes).map((key) => ({
    ...routes[key],
    label: key,
  }));

  const handleMobileMenuClick = (url: string) => {
    navigate(url);
    burgerMenuHandler();
  };

  return (
    <div className={cx(styles['burger-menu'])}>
      <div className={cx(styles['burger-menu-header'])}>
        <div className={cx(styles['burger-menu-close-icon'])}>
          <Button variant="icon" label="" icon={closeMenuIcon} onClick={burgerMenuHandler} />
        </div>
        {/* <WalletInfo account={account} /> */}
      </div>
      {account && (
        <div className={cx(styles['burger-menu-body'])}>
          {mobileMenuItems.map((item) => (
            <Button
              variant="text"
              label={item.label}
              onClick={() => handleMobileMenuClick(item.url)}
              key={item.label}
            />
          ))}
        </div>
      )}
    </div>
  );
}

export { BurgerMenu };

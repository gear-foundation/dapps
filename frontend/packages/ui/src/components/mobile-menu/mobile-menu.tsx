import { PropsWithChildren, Suspense } from 'react';
import Identicon from '@polkadot/react-identicon';
import { motion } from 'framer-motion';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import styles from './mobile-menu.module.css';
import clsx from 'clsx';

export type ClassNameProps = {
  container?: string;
  buttons?: string;
};

type Props = {
  customItems?: {
    icon?: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
    option: JSX.Element;
  }[];
  className?: ClassNameProps;
  onClose(): void;
  onChangeAccountClick(): void;
} & PropsWithChildren;

export function MobileMenu({ children, className, onClose, onChangeAccountClick }: Props) {
  const { account, logout } = useAccount();

  const renderAccount = () => {
    if (!account) return null;
    const { address, meta } = account;

    return (
      <li key={address}>
        <div className={styles.account}>
          <Suspense>
            <Identicon value={address} size={34} theme="polkadot" className={styles.accountIcon} />
          </Suspense>
          <p className={styles.textName}>{meta.name}</p>
        </div>
      </li>
    );
  };

  const handleChangeButtonClick = () => {
    onChangeAccountClick();
    onClose();
  };

  const handleLogoutButtonClick = () => {
    logout();
    onClose();
  };

  return (
    <>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ duration: 0.4 }}
        className={clsx(styles.container, className?.container)}>
        <div className={styles.changeAccount}>
          {children}
          <div>
            <ul className={styles.list}>{renderAccount()}</ul>
          </div>

          <div className={clsx(styles.buttons, className?.buttons)}>
            <Button text="Change account" onClick={handleChangeButtonClick} />
            <Button text="Disconnect" onClick={handleLogoutButtonClick} />
          </div>
        </div>
      </motion.div>
      <div style={{ background: 'black' }} />
    </>
  );
}

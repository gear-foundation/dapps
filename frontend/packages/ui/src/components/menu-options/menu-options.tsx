import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { JSX } from 'react';

import DisconnectSVG from './assets/disconnect.svg?react';
import GridSVG from './assets/grid.svg?react';
import UserSVG from './assets/user.svg?react';
import styles from './menu-options.module.css';

export type ClassNameProps = {
  container?: string;
  nativeIcon?: string;
  item?: string;
};

export type Props = {
  customItems?: {
    icon?: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
    option: JSX.Element;
    key: string;
  }[];
  className?: ClassNameProps;
  onClose?: () => void;
};

export function MenuOptions({ customItems, className, onClose }: Props) {
  const { account, logout } = useAccount();

  const handleLogout = () => {
    logout();
    onClose?.();
  };

  return (
    <div className={clsx(styles.container)}>
      {customItems?.map(({ icon: Icon, option, key }) => (
        <div className={clsx(styles.item)} key={key}>
          {Icon ? <Icon /> : null}
          {option}
        </div>
      ))}
      {customItems?.length && <hr />}
      <a
        href={`https://vara.subscan.io/account/${account?.address}`}
        target="_blank"
        rel="noreferrer"
        className={clsx(styles.item, className?.item)}>
        <UserSVG className={clsx(styles.userSvg, styles.svg, className?.nativeIcon)} />
        <span>View in Blockchain Explorer</span>
      </a>
      <a
        href="https://vara.network/ecosystem"
        target="_blank"
        rel="noreferrer"
        className={clsx(styles.item, className?.item)}>
        <GridSVG className={clsx(styles.svg, className?.nativeIcon)} />
        <span>View other projects on Vara</span>
      </a>
      {account && (
        <>
          <hr />
          <Button
            color="transparent"
            icon={() => <DisconnectSVG className={clsx(styles.svg, className?.nativeIcon)} />}
            text="Disconnect wallet"
            className={clsx(styles.item, styles.disconnectButton, className?.item)}
            onClick={handleLogout}
          />
        </>
      )}
    </div>
  );
}

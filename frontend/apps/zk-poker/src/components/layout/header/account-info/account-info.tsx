import { useApi, useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

import { BurgerMenuIcon, CoinIcon, CrossIcon } from '@/assets/images';
import { Balance, Button } from '@/components/ui';
import { useAccountAvailableBalance } from '@/features/wallet/hooks';

import styles from './account-info.module.scss';

type AccountInfoProps = BaseComponentProps & {
  openWallet: () => void;
  isOpen: boolean;
};

export function AccountInfo({ className, openWallet, isOpen }: AccountInfoProps) {
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const { availableBalance: balance } = useAccountAvailableBalance();
  const formattedBalance = isApiReady && (balance || undefined);

  return (
    <>
      <div className={clsx(styles.wrapper, isOpen && styles.open, className)}>
        {!!account && (
          <>
            {formattedBalance && (
              <Balance
                value={formattedBalance.value}
                unit={formattedBalance.unit}
                className={styles.balance}
                SVG={CoinIcon}
                isLight={isOpen}
              />
            )}

            <Button color="transparent" className={styles.openWallet} onClick={openWallet}>
              {isOpen ? <CrossIcon /> : <BurgerMenuIcon width={24} height={24} />}
            </Button>
          </>
        )}
      </div>
    </>
  );
}

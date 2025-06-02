import { useApi, useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';

import { BurgerMenuIcon, CrossIcon } from '@/assets/images';
import { VaraBalance } from '@/components/ui/balance';
import { Button } from '@/components/ui/button';
import { useAccountAvailableBalance } from '@/features/wallet/hooks';

import styles from './account-info.module.scss';

type AccountInfoProps = BaseComponentProps & {
  openWallet: () => void;
  openConnectWallet: () => void;
  isOpen: boolean;
};

export function AccountInfo({ className, openWallet, openConnectWallet, isOpen }: AccountInfoProps) {
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const { availableBalance: balance } = useAccountAvailableBalance();
  const formattedBalance = isApiReady && (balance || undefined);

  return (
    <>
      <div className={clsx(styles.wrapper, className)}>
        {!!account && (
          <>
            {formattedBalance && (
              <VaraBalance value={formattedBalance.value} unit={formattedBalance.unit} className={styles.balance} />
            )}

            <Button variant="text" className={styles.openWallet} onClick={openWallet}>
              {isOpen ? (
                <CrossIcon />
              ) : (
                <>
                  <BurgerMenuIcon width={24} height={24} />
                </>
              )}
            </Button>
          </>
        )}
        {!account && (
          <Button size="small" className={styles.connectWallet} onClick={openConnectWallet}>
            Connect wallet
          </Button>
        )}
      </div>
    </>
  );
}

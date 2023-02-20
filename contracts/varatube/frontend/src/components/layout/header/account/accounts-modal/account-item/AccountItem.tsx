import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { decodeAddress } from '@gear-js/api';
import { Button } from '@gear-js/ui';
import { ReactComponent as CopySVG } from 'assets/images/icons/copy.svg';
import { AccountButton } from '../../account-button';
import { useCopyToClipboard } from '../hooks';
import styles from './AccountItem.module.scss';

type Props = {
  account: InjectedAccountWithMeta;
  isActive: boolean;
  onClick: (account: InjectedAccountWithMeta) => void;
};

function AccountItem({ account, isActive, onClick }: Props) {
  const copyToClipboard = useCopyToClipboard();

  const handleClick = () => {
    if (isActive) return;

    onClick(account);
  };

  const handleCopyButtonClick = () => {
    const decodedAddress = decodeAddress(account.address);

    copyToClipboard(decodedAddress);
  };

  return (
    <li className={styles.accountItem}>
      <AccountButton
        name={account.meta.name}
        address={account.address}
        isActive={isActive}
        onClick={handleClick}
        block
      />
      <Button icon={CopySVG} color="transparent" onClick={handleCopyButtonClick} />
    </li>
  );
}

export { AccountItem };

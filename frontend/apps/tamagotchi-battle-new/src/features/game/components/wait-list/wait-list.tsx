import { Text } from '@/components';
import { Button } from '@gear-js/vara-ui';
import styles from './wait-list.module.scss';
import { copyToClipboard } from '@/app/utils';
import { getVaraAddress, useAccount, useAlert } from '@gear-js/react-hooks';
import { stringShorten } from '@polkadot/util';
import { CopyIcon } from '../../assets/images';
import clsx from 'clsx';
import { ScrollArea } from '@/components/ui/scroll-area';

type WaitListItem = {
  name: string;
  address: string;
};

type WaitListProps = {
  items: WaitListItem[];
};

const WaitList = ({ items }: WaitListProps) => {
  const alert = useAlert();
  const {account} = useAccount()

  const handleCopyAddress = (value: string) => {
    copyToClipboard({ alert, value });
  };

  return (
    <ScrollArea className={styles.list}>
      {items.map(({ name, address }, index) => {
        // ! TODO: scroll to my on mount
        const isMy = address === account.decodedAddress;

        return (
          <div key={address} className={clsx(styles.item, { [styles.my]: isMy })}>
            <Text size="sm" weight="semibold" className={styles.number}>
              {index + 1}
            </Text>
            <Text size="sm" weight="semibold">
              {name}
            </Text>
            <div className={styles.addressWrapper}>
              <Text size="sm" weight="medium">
                {stringShorten(getVaraAddress(address), 8)}
              </Text>
              <Button
                color="transparent"
                icon={CopyIcon}
                onClick={() => handleCopyAddress(getVaraAddress(address))}></Button>
            </div>
          </div>
        );
      })}
    </ScrollArea>
  );
};

export { WaitList };

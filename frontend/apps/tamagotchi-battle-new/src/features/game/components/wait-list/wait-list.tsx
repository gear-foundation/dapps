import { useEffect, useRef } from 'react';
import clsx from 'clsx';
import { stringShorten } from '@polkadot/util';
import { Button } from '@gear-js/vara-ui';
import { Text } from '@/components';
import { copyToClipboard } from '@/app/utils';
import { getVaraAddress, useAccount, useAlert } from '@gear-js/react-hooks';
import { CopyIcon } from '../../assets/images';
import { ScrollArea } from '@/components/ui/scroll-area';
import styles from './wait-list.module.scss';

type WaitListItem = {
  name: string;
  address: string;
};

type WaitListProps = {
  items: WaitListItem[];
};

const WaitList = ({ items }: WaitListProps) => {
  const alert = useAlert();
  const { account } = useAccount();

  const handleCopyAddress = (value: string) => {
    copyToClipboard({ alert, value });
  };

  const myItemRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    myItemRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  return (
    <ScrollArea className={styles.list}>
      {items.map(({ name, address }, index) => {
        const isMy = address === account.decodedAddress;

        return (
          <div key={address} ref={isMy ? myItemRef : undefined} className={clsx(styles.item, { [styles.my]: isMy })}>
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

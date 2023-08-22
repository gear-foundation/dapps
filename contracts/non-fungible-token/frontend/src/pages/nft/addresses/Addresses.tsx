import { HexString } from '@polkadot/util/types';
import { Button } from '@gear-js/ui';
import { Card } from '../card';
import styles from './Addresses.module.scss';

type Props = {
  list: HexString[];
  onAddressClick: (address: HexString) => void;
  isOwner: boolean;
};

function Addresses({ list, onAddressClick, isOwner }: Props) {
  const getAddresses = () =>
    list.map((address) => (
      <li key={address} className={styles.address}>
        <span className={styles.text}>{address}</span>
        {isOwner && (
          <Button
            text="Revoke approval"
            color="secondary"
            size="small"
            className={styles.button}
            onClick={() => onAddressClick(address)}
          />
        )}
      </li>
    ));

  return (
    <Card heading="Approved addresses">
      <ul className={styles.list}>{getAddresses()}</ul>
    </Card>
  );
}

export { Addresses };

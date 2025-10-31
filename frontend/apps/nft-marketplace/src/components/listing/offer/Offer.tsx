import { HexString } from '@gear-js/api';
import { useAccount, useBalanceFormat } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useState } from 'react';
import { useParams } from 'react-router-dom';

import { useAcceptOfferMessage } from '@/app/utils';
import { ConfirmationModal } from '@/components/modals';

import styles from './Offer.module.scss';

type Props = {
  bidder: string;
  listingOwner: HexString;
  price: string;
};

type Params = {
  id: string;
};

function Offer({ bidder, listingOwner, price }: Props) {
  const { id } = useParams() as Params;
  const { account } = useAccount();

  const { acceptOfferMessage } = useAcceptOfferMessage();
  const { getFormattedBalance } = useBalanceFormat();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const isOwner = account?.decodedAddress === listingOwner;

  const formattedPrice = getFormattedBalance(price).value;

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  const accept = () => {
    void acceptOfferMessage({ tokenId: id, price: BigInt(price) }, { onSuccess: closeModal });
  };

  return (
    <>
      <div className={styles.offer}>
        <div className={styles.info}>
          <p className={styles.bid}>{formattedPrice}</p>
          <p className={styles.bidder}>{bidder}</p>
        </div>
        {isOwner && <Button text="Accept" size="small" onClick={openModal} />}
      </div>

      {isModalOpen && (
        <ConfirmationModal
          heading={`Do you agree to sell the item for ${formattedPrice}?`}
          close={closeModal}
          onSubmit={accept}
        />
      )}
    </>
  );
}

export { Offer };

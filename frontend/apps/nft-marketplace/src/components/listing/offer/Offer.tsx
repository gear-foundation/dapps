import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { HexString } from '@polkadot/util/types';
import { ConfirmationModal } from 'components/modals';
import { ADDRESS } from 'consts';
import { useMarketplaceMessage } from 'hooks';
import { useState } from 'react';
import { useParams } from 'react-router-dom';
import styles from './Offer.module.scss';

type Props = {
  bidder: string;
  listingOwner: HexString;
  price: string;
};

type Params = {
  tokenId: string;
};

function Offer({ bidder, listingOwner, price }: Props) {
  const { tokenId } = useParams() as Params;
  const { account } = useAccount();

  const sendMessage = useMarketplaceMessage();

  const [isModalOpen, setIsModalOpen] = useState(false);

  const isOwner = account?.decodedAddress === listingOwner;

  const openModal = () => {
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
  };

  const accept = () => {
    const payload = { AcceptOffer: { nft_contract_id: ADDRESS.NFT_CONTRACT, token_id: tokenId, price } };

    sendMessage({ payload, onSuccess: closeModal });
  };

  return (
    <>
      <div className={styles.offer}>
        <div className={styles.info}>
          <p className={styles.bid}>{price}</p>
          <p className={styles.bidder}>{bidder}</p>
        </div>
        {isOwner && <Button text="Accept" size="small" onClick={openModal} />}
      </div>

      {isModalOpen && (
        <ConfirmationModal
          heading={`Do you agree to sell the item for ${price}?`}
          close={closeModal}
          onSubmit={accept}
        />
      )}
    </>
  );
}

export { Offer };

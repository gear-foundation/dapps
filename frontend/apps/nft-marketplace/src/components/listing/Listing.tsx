import { useBalanceFormat } from '@gear-js/react-hooks';
import { ReactNode } from 'react';
import { Listing as ListingType } from '@/types';
import { Card } from './card';
import { Offer } from './offer';
import styles from './Listing.module.scss';

type Props = {
  item: ListingType;
  children: ReactNode;
};

function Listing({ children, item }: Props) {
  const { heading, description, owner, currentWinner, price, src, rarity, attrs, offers } = item;
  const isAnyOffer = !!offers?.length;
  const { getFormattedBalance } = useBalanceFormat();

  const getAttributes = () =>
    attrs &&
    Object.keys(attrs).map((attr, index) => (
      // eslint-disable-next-line react/no-array-index-key
      <p key={index} className={styles.text}>{`${attr}: ${attrs![attr]}`}</p>
    ));

  const getOffers = () =>
    offers
      ?.map(({ price: offerPrice, bidder }) => (
        <Offer key={offerPrice} price={offerPrice} bidder={bidder} listingOwner={owner} />
      ))
      .reverse();

  const priceText = price ? getFormattedBalance(price).value : undefined;
  const hasCurrentWinner = currentWinner && Number(currentWinner) !== 0;

  return (
    <>
      <h2 className={styles.heading}>{heading}</h2>
      <div className={styles.listing}>
        <div>
          {priceText && <Card heading="Current price" text={priceText} />}
          <Card heading="Description" text={description} />
          <Card heading="Owner" text={owner} />
          {hasCurrentWinner && <Card heading="Current Winner" text={currentWinner} />}
        </div>
        <div className={styles.main}>
          <div className={styles.imgWrapper}>
            <img src={src} alt="" className={styles.image} />
          </div>
          <div className={styles.buttons}>{children}</div>
        </div>
        <div>
          {rarity && <Card heading="Rarity" text={rarity} />}
          {attrs && <Card heading="Attributes">{getAttributes()}</Card>}

          {offers && <Card heading={isAnyOffer ? 'Offers' : 'No offers'}>{getOffers()}</Card>}
        </div>
      </div>
    </>
  );
}

export { Listing };

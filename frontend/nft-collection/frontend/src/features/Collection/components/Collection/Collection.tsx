import { useState } from 'react';
import { useAtomValue } from 'jotai';
import { CollectionProps } from './Collection.interfaces';
import styles from './Collection.module.scss';
import { cx, shortenString } from '@/utils';
import 'swiper/css';
import 'swiper/css/navigation';
import { Gallery } from '@/components/Gallery';
import { getNotMintedTokens, getTimeFormatFromStateDate } from '../../utils';
import { ReactComponent as IcImage } from '../../assets/images/ic-image-24.svg';
import { ReactComponent as Clocks } from '../../assets/images/watch-later-24px.svg';
import { ReactComponent as UserPlus } from '../../assets/images/user-plus.svg';
import { DescriptionItem } from '@/components';
import { Button, Link } from '@/ui';
import { NftPreview } from '@/features/Nft/components/NftPreview';
import { NFT, YOUR_SPACE } from '@/routes';
import { useCollectionMessage } from '@/hooks';

import { COLLECTIONS } from '../../atoms';
import { ACCOUNT_ATOM } from '@/atoms';

function Collection({ data }: CollectionProps) {
  const { collection, tokens, owner, timeCreation, id: address } = data;
  const { message } = useCollectionMessage(address);
  const account = useAtomValue(ACCOUNT_ATOM);
  const [isMinting, setIsMinting] = useState(false);
  const nftMaxCount = 10;

  const handleMintNft = () => {
    setIsMinting(true);
    const payload = {
      Mint: null,
    };

    message(payload, {
      onSuccess: () => {
        setIsMinting(false);
      },
      onError: () => {
        setIsMinting(false);
      },
    });
  };

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.header)}>
        <div className={cx(styles.title)}>{collection.name}</div>
        <div className={cx(styles.info)}>
          <DescriptionItem
            icon={<IcImage className={cx(styles['svg-image'])} />}
            text={`Minted images: ${`${getNotMintedTokens(tokens).length}/${nftMaxCount}`}`}
          />
          <DescriptionItem
            icon={<Clocks className={cx(styles['svg-image'])} />}
            text={`Created: ${getTimeFormatFromStateDate(timeCreation)}`}
          />
          <DescriptionItem
            icon={<UserPlus className={cx(styles['svg-image'])} />}
            text={`Created by: ${shortenString(owner, 5)}`}
          />
        </div>
        <div className={cx(styles.buttons)}>
          <Button
            variant="primary"
            className={cx(styles.button)}
            label="Mint NFT"
            onClick={handleMintNft}
            disabled={isMinting || tokens.some((token) => token.owner === account?.decodedAddress)}
          />
          <Link to={`${YOUR_SPACE}`}>
            <Button
              variant="primary"
              className={cx(styles.button, styles['button-grey'])}
              label="Creator Space"
              disabled={isMinting}
            />
          </Link>
        </div>
      </div>
      <div className={cx(styles['gallery-wrapper'])}>
        <Gallery
          data={tokens.map((token) => ({
            component: (
              <Link to={`${NFT}/${token.id}`}>
                <NftPreview
                  url={token.medium}
                  name={token.name}
                  collectionName={token.collectionName}
                  owner={token.owner}
                  timeMinted={token.timeMinted || timeCreation}
                />
              </Link>
            ),
            id: `${token.timeMinted}-${token.medium}-${token.owner}`,
          }))}
          emptyText={
            <>
              <span>There are no NFTs here yet.</span>
              <span>Mint your first NFT, and it will be displayed here.</span>
            </>
          }
        />
      </div>
    </div>
  );
}

export { Collection };

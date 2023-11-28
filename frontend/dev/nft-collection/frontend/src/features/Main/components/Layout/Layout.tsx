import moment from 'moment';
import { useAtomValue } from 'jotai';
import { Link } from '@ui';
import styles from './Layout.module.scss';
import { cx } from '@/utils';
import images from '../../assets/images/nft-main.png';
import { Button } from '@/ui';
import { NftPreview } from '@/features/Nft/components/NftPreview';
import { CollectionPreview } from '@/features/Collection/components/CollectionPreview';
import { Swiper } from '@/components/Swiper';
import { COLLECTION, CREATE_COLLECTION, EXPLORE } from '@/routes';
import { COLLECTIONS } from '@/features/Collection/atoms';

function Layout() {
  const collections = useAtomValue(COLLECTIONS);

  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles.content)}>
        <img src={images} alt="nft main" className={cx(styles['nft-main-image'])} />
        <div className={cx(styles.presentation)}>
          <h1 className={cx(styles.title)}>Vara NFT</h1>
          <span className={cx(styles.text)}>Discover Vara NFT Marketplace - Create, Connect, Collect!</span>
          <span className={cx(styles.text)}>
            Your hub for creating and exploring NFTs. Unleash your creativity, connect with fellow creators, and own
            digital assets like never before.
          </span>
          <div className={cx(styles.buttons)}>
            <Link to={CREATE_COLLECTION}>
              <Button label="Create Collection" variant="primary" />
            </Link>
            <Link to={EXPLORE}>
              <Button label="Explore" variant="primary" />
            </Link>
          </div>
        </div>
      </div>
      <div className={cx(styles.collections)}>
        <Swiper
          title="Featured Collections"
          data={Object.keys(collections).map((id) => {
            const collection = collections[id];

            return (
              <Link to={`${COLLECTION}/${id}`}>
                <CollectionPreview collection={collection} />
              </Link>
            );
          })}
          wrapperClass={cx(styles['with-padding'])}
          withNavigation
        />
        <Swiper
          title="Recently Created Collections"
          data={Object.keys(collections)
            .sort((a, b) =>
              moment(collections[b].timeCreation, 'YYYY-M-D HH:mm').diff(
                moment(collections[a].timeCreation, 'YYYY-M-D HH:mm'),
              ),
            )
            .map((id) => {
              const collection = collections[id];

              return (
                <Link to={`${COLLECTION}/${id}`}>
                  <CollectionPreview collection={collection} />
                </Link>
              );
            })}
          wrapperClass={cx(styles['with-padding'])}
          withNavigation
        />
        <Swiper
          title="Recently Minted NFTs"
          data={Object.keys(collections)
            .map((id) => {
              const collection = collections[id];

              return collection.tokens;
            })
            .flat()
            .sort((a, b) => moment(b.timeMinted, 'YYYY-M-D HH:mm').diff(moment(a.timeMinted, 'YYYY-M-D HH:mm')))
            .slice(0, 20)
            .map((token) => (
              <Link to={`${COLLECTION}/${token.id}`}>
                <NftPreview
                  url={token.medium}
                  name={token.name}
                  collectionName={token.collectionName}
                  owner={token.owner}
                  timeMinted={token.timeMinted}
                />
              </Link>
            ))}
          wrapperClass={cx(styles['with-padding'])}
          withNavigation
        />
      </div>
    </div>
  );
}

export { Layout };

import moment from 'moment';
import { useAtomValue } from 'jotai';
import { HexString } from '@polkadot/util/types';
import { useAccount } from '@gear-js/react-hooks';
import { createSearchParams, useLocation, useNavigate, useParams } from 'react-router-dom';
import { ChangeEvent, useEffect, useState } from 'react';
import { cx, getIpfsAddress, shortenString } from '@/utils';
import { useNodeAddress } from '@/features/NodeSwitch/hooks';
import icUserStar from '../../assets/images/ic-user-star-16.svg';
import icUserPlus from '@/features/Collection/assets/images/user-plus.svg';
import clock from '../../assets/images/watch-later-24px.svg';
import labelRounded from '@/features/Collection/assets/images/label-24px-rounded.svg';
import { ReactComponent as BackArrowSVG } from '../../assets/images/back-arrow.svg';
import { getImageUrl } from '../../utils';
import styles from './NFT.module.scss';
import { TransferNFTModal } from '../TransferNftModal';

import { COLLECTIONS } from '@/features/Collection/atoms';
import { NftSpec } from '../NftSpec';

type Params = {
  collectionId: HexString;
  nftId: string;
};

function NFT() {
  const collections = useAtomValue(COLLECTIONS);
  const { collectionId, nftId } = useParams() as Params;
  const { account } = useAccount();
  const { pathname } = useLocation();
  const navigate = useNavigate();

  const { isTestnet } = useNodeAddress();
  const nft = collections[collectionId].tokens[Number(nftId)];
  const { name, description, owner, medium, collectionName, timeMinted } = nft || {};
  const [details, setDetails] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    if (!medium) return;

    const isIPFSHash = !Array.isArray(medium);

    if (isIPFSHash) {
      const url = getIpfsAddress(medium);

      fetch(url)
        .then((response) => response.json())
        .then((result) => setDetails(result));
    } else {
      setDetails(medium);
    }
  }, [medium]);

  useEffect(() => {
    setSearchQuery('');
  }, [pathname]);

  const getDetails = () =>
    details
      .filter((detail) => {
        const lowerCaseDetail = detail.toLocaleLowerCase();
        const lowerCaseQuery = searchQuery.toLocaleLowerCase();

        return lowerCaseDetail.includes(lowerCaseQuery);
      })
      .map((detail) => (
        <li key={detail} className={styles.detail}>
          <p>{detail}</p>
        </li>
      ));

  const handleSearchInputChange = ({ target }: ChangeEvent<HTMLInputElement>) => setSearchQuery(target.value);

  const handleOwnerButtonClick = (ownerAddress: string) =>
    navigate({ pathname: '/list', search: createSearchParams({ query: ownerAddress || '' }).toString() });

  const handleBackButtonClick = () => navigate(-1);

  const [isTransferModalOpen, setIsTransferModalOpen] = useState(false);
  const openTransferModal = () => setIsTransferModalOpen(true);
  const closeTransferModal = () => setIsTransferModalOpen(false);

  return (
    <>
      <div className={cx(styles.container)}>
        {nft ? (
          <>
            <div className={cx(styles.wrapper)}>
              <div className={styles.innerContainer}>
                <div className={styles.imageWrapper}>
                  <img src={getImageUrl(nft.medium)} alt="" />
                </div>
              </div>

              <div className={styles.innerContainer}>
                <h2 className={styles.name}>{name}</h2>
                <p className={styles.collection}>{collectionName}</p>
                <p className={styles.description}>{description}</p>

                <div className={styles.buttons}>
                  {!isTestnet && account?.decodedAddress === owner && (
                    <button type="button" className={styles.transferButton} onClick={openTransferModal}>
                      Transfer
                    </button>
                  )}
                </div>
              </div>
            </div>
            <div className={styles.footerWrapper}>
              <footer className={styles.footer}>
                <NftSpec title="Token Standart:" value="gNFT" icon={labelRounded} />
                <NftSpec
                  title="Minted:"
                  value={moment(timeMinted, 'YYYY-M-D HH:mm').format('DD.MM.YYYY')}
                  icon={clock}
                />
                <div className={cx(styles['spec-wrapper'])}>
                  <NftSpec
                    title="Created by:"
                    value={shortenString(collections[collectionId].owner, 3)}
                    icon={icUserPlus}
                  />
                  <button className={cx(styles['view-button'])} onClick={() => handleOwnerButtonClick(owner)}>
                    View
                  </button>
                </div>
                <div className={cx(styles['spec-wrapper'])}>
                  <NftSpec title="Owned by:" value={shortenString(owner || '', 3)} icon={icUserStar} />
                  <button
                    className={cx(styles['view-button'])}
                    disabled={!owner}
                    onClick={() => handleOwnerButtonClick(owner)}>
                    View
                  </button>
                </div>
              </footer>
            </div>
          </>
        ) : (
          <p>
            NFT with id {nftId} in {collectionId} contract not found.
          </p>
        )}
      </div>

      {isTransferModalOpen && <TransferNFTModal onClose={closeTransferModal} />}
    </>
  );
}

export { NFT };

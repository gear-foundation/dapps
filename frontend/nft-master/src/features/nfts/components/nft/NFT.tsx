import { HexString } from '@polkadot/util/types';
import { useNavigate, useParams } from 'react-router-dom';
import { ChangeEvent, useEffect, useState } from 'react';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as SearchSVG } from '../../assets/search.svg';
import { ReactComponent as BackArrowSVG } from '../../assets/back-arrow.svg';
import { useNFTsState } from '../../hooks';
import styles from './NFT.module.scss';

type Params = {
  programId: HexString;
  id: string;
};

function NFT() {
  const { programId, id } = useParams() as Params;

  const nftStates = useNFTsState();

  const { tokens, collection } = nftStates?.find(({ programId: _programId }) => _programId === programId) || {};
  const nftState = tokens?.find(([tokenId]) => tokenId === id);
  const nft = nftState?.[1];

  const collectionName = collection?.name;
  const { name, owner, mediaUrl, description, attribUrl } = nft || {};

  const [details, setDetails] = useState<string[]>([]);

  const [searchQuery, setSearchQuery] = useState('');
  const navigate = useNavigate();

  const src = mediaUrl ? getIpfsAddress(mediaUrl) : '';

  useEffect(() => {
    if (!attribUrl) return;

    const url = getIpfsAddress(attribUrl);

    fetch(url)
      .then((response) => response.json())
      .then((result) => setDetails(result));
  }, [attribUrl]);

  const getDetails = () =>
    details
      .filter((detail) => detail.includes(searchQuery))
      .map((detail) => (
        <li key={detail} className={styles.detail}>
          {detail}
        </li>
      ));

  const handleSearchInputChange = ({ target }: ChangeEvent<HTMLInputElement>) => setSearchQuery(target.value);
  const handleBackButtonClick = () => navigate(-1);

  return (
    <Container className={styles.container}>
      <div>
        <div className={styles.imageWrapper}>
          <img src={src} alt="" />
        </div>

        <div className={styles.footerWrapper}>
          <footer className={styles.footer}>
            <p className={styles.owner}>
              <span className={styles.ownerHeading}>Owner:</span>
              <span className={styles.ownerText}>{owner}</span>
            </p>
          </footer>
        </div>
      </div>

      <div>
        <div>
          <h2 className={styles.name}>{name}</h2>
          <p className={styles.collection}>{collectionName}</p>
          <p className={styles.description}>{description}</p>

          <div>
            <header className={styles.header}>
              {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
              <label htmlFor="search" className={styles.label}>
                NFT Details:
              </label>

              <div className={styles.inputWrapper}>
                <SearchSVG />
                <input type="text" placeholder="Search" id="search" onChange={handleSearchInputChange} />
              </div>
            </header>

            <ul className={styles.details}>{getDetails()}</ul>
          </div>
        </div>

        <button type="button" className={styles.backButton} onClick={handleBackButtonClick}>
          <BackArrowSVG />
          <span>Back</span>
        </button>
      </div>
    </Container>
  );
}

export { NFT };

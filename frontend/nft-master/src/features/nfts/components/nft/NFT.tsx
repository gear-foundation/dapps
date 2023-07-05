import { HexString } from '@polkadot/util/types';
import { createSearchParams, useLocation, useNavigate, useParams } from 'react-router-dom';
import { ChangeEvent, useEffect, useState } from 'react';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as SearchSVG } from '../../assets/search.svg';
import { ReactComponent as BackArrowSVG } from '../../assets/back-arrow.svg';
import { useNFTs } from '../../hooks';
import { getImageUrl } from '../../utils';
import styles from './NFT.module.scss';

type Params = {
  programId: HexString;
  id: string;
};

function NFT() {
  const { programId, id } = useParams() as Params;
  const { pathname } = useLocation();
  const navigate = useNavigate();

  const { nfts } = useNFTs();
  const nft = nfts.find((item) => item.programId === programId && item.id === id);
  const { name, collection, description, owner, attribUrl } = nft || {};

  const [details, setDetails] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    if (!attribUrl) return;

    const isIPFSHash = !Array.isArray(attribUrl);

    if (isIPFSHash) {
      const url = getIpfsAddress(attribUrl);

      fetch(url)
        .then((response) => response.json())
        .then((result) => setDetails(result));
    } else {
      setDetails(attribUrl);
    }
  }, [attribUrl]);

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

  const handleOwnerButtonClick = () =>
    navigate({ pathname: '/list', search: createSearchParams({ query: owner || '' }).toString() });

  const handleBackButtonClick = () => navigate(-1);

  return (
    <Container className={styles.container}>
      {nft ? (
        <>
          <div>
            <div className={styles.imageWrapper}>
              <img src={getImageUrl(nft.mediaUrl)} alt="" />
            </div>

            <div className={styles.footerWrapper}>
              <footer className={styles.footer}>
                <p className={styles.owner}>
                  <span className={styles.ownerHeading}>Owner:</span>
                  <span className={styles.ownerText}>{owner}</span>
                </p>

                <button type="button" className={styles.ownerButton} onClick={handleOwnerButtonClick}>
                  View NFTs
                </button>
              </footer>
            </div>
          </div>

          <div>
            <h2 className={styles.name}>{name}</h2>
            <p className={styles.collection}>{collection}</p>
            <p className={styles.description}>{description}</p>

            {attribUrl && (
              <div>
                <header className={styles.header}>
                  {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
                  <label htmlFor="search" className={styles.label}>
                    NFT Details:
                  </label>

                  <div className={styles.inputWrapper}>
                    <SearchSVG />
                    <input
                      type="text"
                      placeholder="Search"
                      id="search"
                      value={searchQuery}
                      onChange={handleSearchInputChange}
                    />
                  </div>
                </header>

                <ul className={styles.details}>{getDetails()}</ul>
              </div>
            )}

            <button type="button" className={styles.backButton} onClick={handleBackButtonClick}>
              <BackArrowSVG />
              <span>Back</span>
            </button>
          </div>
        </>
      ) : (
        <p>
          NFT with id {id} in {programId} contract not found.
        </p>
      )}
    </Container>
  );
}

export { NFT };

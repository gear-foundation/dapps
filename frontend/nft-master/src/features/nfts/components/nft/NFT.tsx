import { useLocation, useNavigate } from 'react-router-dom';
import { ChangeEvent, useEffect, useState } from 'react';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as SearchSVG } from '../../assets/search.svg';
import { ReactComponent as BackArrowSVG } from '../../assets/back-arrow.svg';
import { NFT as NFTType } from '../../types';
import styles from './NFT.module.scss';

type Props = {
  item: NFTType;
};

function NFT({ item }: Props) {
  const { pathname } = useLocation();
  const navigate = useNavigate();

  const [details, setDetails] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  const { name, collection, description, owner, mediaUrl, attribUrl } = item;
  const src = getIpfsAddress(mediaUrl);

  useEffect(() => {
    const url = getIpfsAddress(attribUrl);

    fetch(url)
      .then((response) => response.json())
      .then((result) => setDetails(result));
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
          <p className={styles.collection}>{collection}</p>
          <p className={styles.description}>{description}</p>

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

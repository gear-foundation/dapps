import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';
import { useKeenSlider } from 'keen-slider/react';
import clsx from 'clsx';
import { Container } from 'components';
import { getImageUrl } from '../../utils';
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg';
import { useNFTSearch, useNFTs } from '../../hooks';
import styles from './NFTs.module.scss';

type Props = {
  slider?: boolean;
};

function NFTs({ slider }: Props) {
  const { nfts } = useNFTs();
  const { searchQuery, decodedQueryAddress } = useNFTSearch();
  const { account } = useAccount();

  const filteredNFTs = nfts.filter(({ name, owner }) =>
    searchQuery
      ? name.toLocaleLowerCase().includes(searchQuery.toLocaleLowerCase()) ||
        (decodedQueryAddress && owner === decodedQueryAddress)
      : owner === account?.decodedAddress,
  );

  const nftsCount = filteredNFTs.length;
  const isAnyNFT = nftsCount > 0;
  const middleNFTIndex = Math.floor(nftsCount / 2);

  const [sliderRef, sliderApiRef] = useKeenSlider({
    slides: { perView: 4, spacing: 30, origin: 'center' },
    initial: nftsCount < 4 ? middleNFTIndex : 2,
    breakpoints: {
      '(max-width: 1200px)': {
        slides: { perView: 3.5, spacing: 30, origin: 'center' },
        initial: nftsCount < 4 ? middleNFTIndex : 2,
      },
      '(max-width: 1080px)': {
        slides: { perView: 2.5, spacing: 30, origin: 'center' },
        initial: nftsCount < 3 ? middleNFTIndex : 1,
      },
      '(max-width: 768px)': {
        slides: { perView: 1.75, spacing: 9, origin: 'center' },
        initial: nftsCount < 3 ? middleNFTIndex : 1,
      },
      '(max-width: 576px)': {
        slides: { perView: 1.1, spacing: 9, origin: 'center' },
        initial: nftsCount < 3 ? middleNFTIndex : 1,
      },
    },
  });

  const prevSlide = () => sliderApiRef.current?.prev();
  const nextSlide = () => sliderApiRef.current?.next();

  const getNFTs = () =>
    filteredNFTs.map(({ id, programId, name, owner, mediaUrl, collection }) => {
      const style = { backgroundImage: `url(${getImageUrl(mediaUrl)})` };
      const to = `/${programId}/${id}`;
      const className = clsx(styles.nft, slider && 'keen-slider__slide');

      return (
        <li key={to} className={className}>
          <header>
            <p className={styles.collection}>{collection}</p>
            <p className={styles.name}>{name}</p>
          </header>

          <div className={styles.media} style={style}>
            <footer className={styles.footer}>
              <p className={styles.owner}>
                <span className={styles.ownerHeading}>Owner:</span>
                <span className={styles.ownerText}>{owner}</span>
              </p>

              <Link to={to} className={styles.link}>
                View More
              </Link>
            </footer>
          </div>
        </li>
      );
    });

  return (
    <div>
      {isAnyNFT ? (
        <>
          <Container>
            <header className={styles.header}>
              <h3 className={styles.heading}>{searchQuery ? 'Search' : 'My'} NFTs:</h3>

              {slider && (
                <div>
                  <button type="button" className={styles.leftButton} onClick={prevSlide}>
                    <ArrowLeftSVG />
                  </button>

                  <button type="button" className={styles.rightButton} onClick={nextSlide}>
                    <ArrowLeftSVG />
                  </button>
                </div>
              )}
            </header>
          </Container>

          {slider ? (
            <ul className="keen-slider" ref={sliderRef}>
              {getNFTs()}
            </ul>
          ) : (
            <Container>
              <ul className={styles.list}>{getNFTs()}</ul>
            </Container>
          )}
        </>
      ) : (
        <div className={styles.placeholder}>
          <p className={styles.placeholderHeading}>No NFTs found {!searchQuery && 'for this account'}</p>
          <p className={styles.placeholderText}>
            Suggest to specify custom contract address or switch to another network
          </p>
        </div>
      )}
    </div>
  );
}

export { NFTs };

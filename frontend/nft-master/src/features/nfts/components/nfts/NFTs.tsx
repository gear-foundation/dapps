import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';
import { useKeenSlider } from 'keen-slider/react';
import clsx from 'clsx';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg';
import { useNFTs } from '../../hooks';
import styles from './NFTs.module.scss';

type Props = {
  slider?: boolean;
};

function NFTs({ slider }: Props) {
  const { account } = useAccount();

  const nfts = useNFTs();
  const ownerNFTs = nfts.filter(({ owner }) => owner === account?.decodedAddress);
  const ownerNFTsCount = ownerNFTs.length;
  const isAnyNFT = ownerNFTsCount > 0;

  const [sliderRef, sliderApiRef] = useKeenSlider({
    slides: { perView: 4, spacing: 30, origin: 'center' },
    initial: ownerNFTsCount < 4 ? Math.floor(ownerNFTsCount / 2) : 2,
  });

  const prevSlide = () => sliderApiRef.current?.prev();
  const nextSlide = () => sliderApiRef.current?.next();

  const getNFTs = () =>
    ownerNFTs.map(({ id, programId, name, owner, mediaUrl, collection }) => {
      const style = { backgroundImage: `url(${getIpfsAddress(mediaUrl)})` };
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
              <h3 className={styles.heading}>My NFTs:</h3>

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
          <p className={styles.placeholderHeading}>No NFTs found for this account</p>
          <p className={styles.placeholderText}>
            Suggest to specify custom contract address or switch to another network
          </p>
        </div>
      )}
    </div>
  );
}

export { NFTs };

import { Link } from 'react-router-dom';
import useEmblaCarousel from 'embla-carousel-react';
import clsx from 'clsx';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg';
import { useNFTsState } from '../../hooks';
import styles from './NFTs.module.scss';

type Props = {
  slider?: boolean;
};

function NFTs({ slider }: Props) {
  const nftStates = useNFTsState();
  const [emblaRef, emblaApi] = useEmblaCarousel({ align: 'center', loop: true });

  const prevSlide = () => emblaApi?.scrollPrev();
  const nextSlide = () => emblaApi?.scrollNext();

  const getNFTs = () =>
    nftStates?.map(({ tokens, collection, programId }) =>
      tokens.map(([id, token]) => {
        const collectionName = collection.name;
        const { name, mediaUrl, owner } = token;

        const style = { backgroundImage: `url(${getIpfsAddress(mediaUrl)})` };
        const to = `/${programId}/${id}`;

        return (
          <li key={id} className={clsx(styles.nft, slider && styles.emblaSlide)}>
            <header>
              <p className={styles.collection}>{collectionName}</p>
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
      }),
    );

  const isAnyNFT = !!nftStates?.length;

  return (
    <div>
      {isAnyNFT ? (
        <>
          <Container>
            <header className={styles.header}>
              <h3 className={styles.heading}>NFTs:</h3>

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
            <div className={styles.embla} ref={emblaRef}>
              <ul className={styles.emblaContainer}>{getNFTs()}</ul>
            </div>
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

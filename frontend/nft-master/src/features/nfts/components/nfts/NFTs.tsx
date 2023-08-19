import { useAccount } from '@gear-js/react-hooks';
import { Link } from 'react-router-dom';
import { useKeenSlider } from 'keen-slider/react';
import clsx from 'clsx';
import { buttonVariants, Container } from 'components';
import { useNodeAddress } from 'features/node-switch';
import SmokeImage from 'features/nfts/assets/smoke.webp';
import { getImageUrl } from '../../utils';
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg';
import { useNFTSearch, useNFTs, useTestnetNFT } from '../../hooks';
import styles from './NFTs.module.scss';

type Props = {
  slider?: boolean;
};

function NFTs({ slider }: Props) {
  const { nfts } = useNFTs();
  const { searchQuery, decodedQueryAddress } = useNFTSearch();
  const { account } = useAccount();

  const { isTestnet } = useNodeAddress();
  const { mintTestnetNFT, isTestnetNFTMintAvailable, isMinting } = useTestnetNFT();

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

              <Link to={to} className={buttonVariants({ size: 'sm', className: styles.link })}>
                View More
              </Link>
            </footer>
          </div>
        </li>
      );
    });

  return (
    <div className={styles.wrapper}>
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

          <div className="">
            <picture>
              <img src={SmokeImage} alt="Smoke" className={styles.background} />
            </picture>
          </div>

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
          {isTestnet && !searchQuery ? (
            <>
              {isTestnetNFTMintAvailable && (
                <>
                  <p className={styles.placeholderHeading}>You don&apos;t have NFT yet</p>
                  <p className={styles.placeholderText}>To obtain your NFT, click the &quot;Mint NFT&quot; button.</p>
                  <button type="button" onClick={mintTestnetNFT} className={styles.button} disabled={isMinting}>
                    Mint NFT
                  </button>
                </>
              )}

              {!isTestnetNFTMintAvailable && (
                <>
                  <p className={styles.placeholderHeading}>You are currently not part of the Vara Network Testnet.</p>
                  <p className={styles.placeholderText}>
                    More information can be found in our <br />
                    <a href="https://discord.com/invite/7BQznC9uD9" target="_blank" rel="noreferrer">
                      Discord
                    </a>{' '}
                    and{' '}
                    <a href="https://t.me/VaraNetwork_Global" target="_blank" rel="noreferrer">
                      Telegram
                    </a>
                    .
                  </p>
                </>
              )}
            </>
          ) : (
            <>
              <p className={styles.placeholderHeading}>No NFTs found {!searchQuery && 'for this account'}</p>
              <p className={styles.placeholderText}>
                Please provide the custom contract address or&nbsp;switch to another network.
              </p>
            </>
          )}
        </div>
      )}
    </div>
  );
}

export { NFTs };

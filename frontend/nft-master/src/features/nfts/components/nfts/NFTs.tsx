import { useQuery } from 'urql'
import { useAccount } from '@gear-js/react-hooks'
import { Link, Navigate, useNavigate } from 'react-router-dom'
import { useKeenSlider } from 'keen-slider/react'
import clsx from 'clsx'
import { Button, buttonVariants, Container, Loader } from 'components'
import { useCheckBalance } from 'hooks'
import { NFT } from 'features/nfts/types'
import { GetNftsByNameQuery } from 'features/nfts/queries'
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg'
import { useNFTSearch, useNFTs, useMintNFT } from '../../hooks'
import styles from './NFTs.module.scss'

type Props = {
  slider?: boolean
}

function NFTs({ slider }: Props) {
  const { nfts, getImageUrl } = useNFTs()
  const { searchQuery } = useNFTSearch()
  const { account } = useAccount()
  const { getIsLowBalance } = useCheckBalance()
  const navigate = useNavigate()

  const { mintNFT, isMintingAvailable, isMinting } = useMintNFT()

  const [result] = useQuery({
    query: GetNftsByNameQuery,
    variables: { search_query: searchQuery || null },
  })

  const { data: searchedData, fetching: searching, error } = result

  const filteredNFTs = searchQuery ? (searchedData?.nfts as NFT[]) : nfts

  const nftsCount = filteredNFTs?.length
  const isAnyNFT = nftsCount > 0
  const middleNFTIndex = Math.floor(nftsCount / 2)

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
  })

  const prevSlide = () => sliderApiRef.current?.prev()
  const nextSlide = () => sliderApiRef.current?.next()

  const getNFTs = () =>
    filteredNFTs.map(({ name, owner, mediaUrl, collection }) => {
      const style = { backgroundImage: `url(${getImageUrl(mediaUrl)})` }
      const to = `/${owner.id}`
      const className = clsx(styles.nft, slider && 'keen-slider__slide')

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
                <span className={styles.ownerText}>{owner.id}</span>
              </p>

              <Link
                to={to}
                className={buttonVariants({
                  size: 'sm',
                  className: styles.link,
                })}
              >
                View More
              </Link>
            </footer>
          </div>
        </li>
      )
    })

  if (!account && !searchQuery) {
    return <Navigate to="/" replace />
  }

  const isLoading = searching

  return (
    <div className={styles.wrapper}>
      {isLoading ? (
        <Loader />
      ) : (
        <div className={styles.content}>
          {isAnyNFT ? (
            <>
              <Container>
                <header className={styles.header}>
                  <h3 className={styles.heading}>
                    {searchQuery ? 'Search' : 'My'} NFTs:
                  </h3>

                  {slider && (
                    <div>
                      <button
                        type="button"
                        className={styles.leftButton}
                        onClick={prevSlide}
                      >
                        <ArrowLeftSVG />
                      </button>

                      <button
                        type="button"
                        className={styles.rightButton}
                        onClick={nextSlide}
                      >
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
              {!searchQuery ? (
                <>
                  {(isMinting || isMintingAvailable) && (
                    <p className={styles.placeholderHeading}>
                      You don&apos;t have NFTs
                    </p>
                  )}

                  {!isMinting && !isMintingAvailable && (
                    <>
                      <p className={styles.placeholderHeading}>
                        There is nothing here yet
                      </p>
                      <p
                        className={clsx(
                          styles.placeholderText,
                          styles.placeholderTextMax
                        )}
                      >
                        Due to high system load, it may take some time to
                        process your NFT. Please try again in several minutes or
                        refresh the&nbsp;page.
                        <br />
                        If you are not currently part of the Vara Network
                        Testnet, click on &quot;Register&quot;.
                      </p>
                      <div className={styles.placeholder__actions}>
                        <Button onClick={() => navigate(0)}>Reload page</Button>
                        <a
                          href="https://gear-faucet.vara-network.io/links/6a8caca9-8833-49ee-ba06-55f5943d770f"
                          target="_blank"
                          rel="noreferrer"
                          className={buttonVariants({ variant: 'black' })}
                        >
                          Register
                        </a>
                      </div>
                    </>
                  )}
                </>
              ) : (
                <>
                  <p className={styles.placeholderHeading}>
                    No NFTs found {!searchQuery && 'for this account'}
                  </p>
                  <p className={styles.placeholderText}>
                    Please provide the custom contract address or&nbsp;switch to
                    another network.
                  </p>
                </>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  )
}

export { NFTs }

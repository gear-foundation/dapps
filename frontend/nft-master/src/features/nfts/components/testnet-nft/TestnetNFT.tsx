import { useAccount, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useMemo } from 'react';
import { getProgramMetadata } from '@gear-js/api';
import { Navigate, useNavigate } from 'react-router-dom';
import { Container } from 'components';
import { ReactComponent as BackArrowSVG } from '../../assets/back-arrow.svg';
import { useNFTs } from '../../hooks';
import { TESTNET_NFT_CONTRACT_ADDRESS } from '../../consts';
import { NFTContractState } from '../../types';
import nftStyles from '../nft/NFT.module.scss';
import styles from './TestnetNFT.module.scss';

type TestnetNFTState = NFTContractState & {
  constraints: {
    authorizedMinters: HexString[];
  };
};

function TestnetNFT() {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { NFTContracts, nfts } = useNFTs();
  const navigate = useNavigate();

  const contract = NFTContracts.find(([address]) => address === TESTNET_NFT_CONTRACT_ADDRESS);
  const nft = nfts.find(
    ({ programId, owner }) => programId === TESTNET_NFT_CONTRACT_ADDRESS && owner === decodedAddress,
  );
  const metaRaw = contract?.[1];
  const metaHex = metaRaw ? (`0x${metaRaw}` as HexString) : undefined;

  const metadata = useMemo(() => (metaHex ? getProgramMetadata(metaHex) : undefined), [metaHex]);

  // TODO: better to obtain state from useNFTs to not read state twice,
  // however current implementation return only list of tokens
  const { state } = useReadFullState<TestnetNFTState>(TESTNET_NFT_CONTRACT_ADDRESS, metadata);
  const authorizedMinters = state?.constraints.authorizedMinters;
  const isAccountAuthorized = !!authorizedMinters?.find((address) => address === decodedAddress);

  const sendMessage = useSendMessage(TESTNET_NFT_CONTRACT_ADDRESS, metadata);

  const mint = () => sendMessage({ Mint: null });

  const handleBackButtonClick = () => navigate(-1);

  return nft ? (
    <Navigate to={`/${TESTNET_NFT_CONTRACT_ADDRESS}/${nft.id}`} />
  ) : (
    <>
      {state && (
        <Container className={nftStyles.container}>
          <div>
            <div className={styles.wrapper}>
              <div className={styles.nft}>
                {isAccountAuthorized && (
                  <>
                    <h3 className={styles.heading}>You don&apos;t have NFT yet</h3>
                    <p className={styles.text}>To obtain your NFT, click the &quot;Mint NFT&quot; button.</p>
                    <button type="button" onClick={mint} className={styles.button}>
                      Mint NFT
                    </button>
                  </>
                )}

                {!isAccountAuthorized && (
                  <>
                    <h3 className={styles.heading}>You are currently not part of the Vara Network Testnet.</h3>
                    <p>
                      More information can be found in our{' '}
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
              </div>
            </div>
          </div>

          <div>
            <h2 className={nftStyles.name}>VARA Testnet NFT</h2>
            <p className={nftStyles.collection}>Vara Testnet Launch collection</p>
            <p className={nftStyles.description}>
              It is a collection of digital assets created on the Vara blockchain and traded in the non-fungible token
              (NFT) format. Each token represents a unique character - with unique characteristics and attributes.
            </p>

            <button type="button" className={nftStyles.backButton} onClick={handleBackButtonClick}>
              <BackArrowSVG />
              <span>Back</span>
            </button>
          </div>
        </Container>
      )}

      {!state && null}
    </>
  );
}

export { TestnetNFT };

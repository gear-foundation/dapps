import { useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useMemo } from 'react';
import { getProgramMetadata } from '@gear-js/api';
import { Navigate, useNavigate } from 'react-router-dom';
import { Container } from 'components';
import { ReactComponent as BackArrowSVG } from '../../assets/back-arrow.svg';
import { useNFTs } from '../../hooks';
import { TESTNET_NFT_CONTRACT_ADDRESS } from '../../consts';
import nftStyles from '../nft/NFT.module.scss';
import styles from './TestnetNFT.module.scss';

function TestnetNFT() {
  const { NFTContracts, nfts } = useNFTs();
  const navigate = useNavigate();

  const contract = NFTContracts.find(([address]) => address === TESTNET_NFT_CONTRACT_ADDRESS);
  const nft = nfts.find(({ programId }) => programId === TESTNET_NFT_CONTRACT_ADDRESS);
  const metaRaw = contract?.[1];
  const metaHex = metaRaw ? (`0x${metaRaw}` as HexString) : undefined;

  const metadata = useMemo(() => (metaHex ? getProgramMetadata(metaHex) : undefined), [metaHex]);
  const sendMessage = useSendMessage(TESTNET_NFT_CONTRACT_ADDRESS, metadata);

  const mint = () => sendMessage({ MintReferral: { transaction_id: Math.floor(Math.random() * 10000) } });

  const handleBackButtonClick = () => navigate(-1);

  return nft ? (
    <Navigate to={`/${TESTNET_NFT_CONTRACT_ADDRESS}/${nft.id}`} />
  ) : (
    <Container className={nftStyles.container}>
      <div>
        <div className={styles.wrapper}>
          <div className={styles.nft}>
            <h3 className={styles.heading}>You don&apos;t have NFT yet</h3>
            <p className={styles.text}>To obtain your NFT, click the &quot;Mint NFT&quot; button.</p>

            <button type="button" onClick={mint} className={styles.button}>
              Mint NFT
            </button>
          </div>
        </div>
      </div>

      <div>
        <h2 className={nftStyles.name}>My Testnet NFT</h2>
        <p className={nftStyles.collection}>Vara Testnet Launch collection</p>
        <p className={nftStyles.description}>
          It is a collection of digital assets created on the Vara blockchain and traded in the non-fungible token (NFT)
          format. Each token represents a unique character - with unique characteristics and attributes.
        </p>

        <button type="button" className={nftStyles.backButton} onClick={handleBackButtonClick}>
          <BackArrowSVG />
          <span>Back</span>
        </button>
      </div>
    </Container>
  );
}

export { TestnetNFT };

import { Welcome } from 'features/welcome';
import { NFTs } from 'features/nfts';
import styles from './Home.module.scss';

function Home() {
  return (
    <>
      <Welcome />
      <NFTs slider />
    </>
  );
}

export { Home };

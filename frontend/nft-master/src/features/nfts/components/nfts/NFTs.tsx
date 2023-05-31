import { getIpfsAddress } from 'utils';
import { Link } from 'react-router-dom';
import styles from './NFTs.module.scss';

const LIST = [
  {
    id: '0',
    collection: 'collection',
    name: 'name',
    owner: 'owner',
    media: 'QmcXwaEzSrhjrnXGYxqv2ce3DXz2GDnXv1Z1V7mpkcEYfE',
  },
  {
    id: '1',
    collection: 'collection',
    name: 'name',
    owner: 'owner',
    media: 'QmcXwaEzSrhjrnXGYxqv2ce3DXz2GDnXv1Z1V7mpkcEYfE',
  },
  {
    id: '2',
    collection: 'collection',
    name: 'name',
    owner: 'owner',
    media: 'QmcXwaEzSrhjrnXGYxqv2ce3DXz2GDnXv1Z1V7mpkcEYfE',
  },
  {
    id: '3',
    collection: 'collection',
    name: 'name',
    owner: 'owner',
    media: 'QmcXwaEzSrhjrnXGYxqv2ce3DXz2GDnXv1Z1V7mpkcEYfE',
  },
  {
    id: '4',
    collection: 'collection',
    name: 'name',
    owner: 'owner',
    media: 'QmcXwaEzSrhjrnXGYxqv2ce3DXz2GDnXv1Z1V7mpkcEYfE',
  },
];

function NFTs() {
  const getNFTs = () =>
    LIST.map(({ id, collection, name, owner, media }) => {
      const style = { backgroundImage: `url(${getIpfsAddress(media)})` };
      const to = `/nft/${id}`;

      return (
        <li key={id} className={styles.nft}>
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
      <h3 className={styles.heading}>NFTs:</h3>
      <ul className={styles.list}>{getNFTs()}</ul>
    </div>
  );
}

export { NFTs };

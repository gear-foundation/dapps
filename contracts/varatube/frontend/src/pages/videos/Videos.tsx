import { Heading, Loader } from 'components';
import { Link } from 'react-router-dom';
import { ADDRESS } from 'consts';
import { useSubscription } from 'hooks';
import styles from './Videos.module.scss';

const names = [
  'Trash Panda',
  'Raccoon & Beans (R&B)',
  'Pet Me',
  'Lovely Nap',
  "Don't Play w/ Me",
  'Aww',
  'is that even a cat??',
  'Lil Boi',
];

const description =
  'some random description using random words, some random description using random words, some random description using random words, some random description using random words';

const CIDs = [
  { id: 'QmWxkTjf1tX1wfCaocBH2XMDpLSF3Tw8PEzKxn4xxsGjCx', name: 'Trash Panda' },
  { id: 'QmeUgZ8b4RD2DsrEi756YwfbRynyPXZyKEJk2pw8ZHATn3', name: 'Raccoon & Beans (R&B)' },
  { id: 'QmZutgsmUSLUDY6LhEuQFEFAW9AoJ2zmUgQMsxndHNSYRf', name: 'Pet Me' },
  { id: 'QmQKboWyCaqct3PXPDhpvsKCvse1eqRacpErkq6BoFBiiB', name: 'Lovely Nap' },
  { id: 'QmXqJZEZ55UYZBY9Bf67a614DK47xiNakbuEP5XbXFyB4r', name: "Don't Play w/ Me" },
  { id: 'QmbYBTHBTLX8mmAFhhvkdXUt4ZtfwZFPQeJFMFhADoc9Lw', name: 'Aww' },
  { id: 'QmPMo6bEMuLHGU2qvDbJuxHJS2UaDeuZbehshC5BDEvMjW', name: 'is that even a cat??' },
  { id: 'QmP7EUM7AbA1Cshox9EqJ8Ux21C3XJTU4owaUF79KQiRDV', name: 'Lil Boi' },
];

function Videos() {
  const isAnyVideo = CIDs.length > 0;

  const isReady = useSubscription();

  const getVideos = () =>
    CIDs.map((cid) => (
      <li key={cid.id}>
        <Link to={`/video/${cid.id}`} className={styles.video}>
          <h3 className={styles.heading}>{cid.name}</h3>

          <div className={styles.videoWrapper}>
            {/* eslint-disable-next-line jsx-a11y/media-has-caption */}
            <video>
              <source src={`${ADDRESS.IPFS_GATEWAY}${cid.id}`} type="video/mp4" />
            </video>
          </div>

          <p className={styles.description}>{description}</p>
        </Link>
      </li>
    ));

  return isReady ? (
    <>
      <Heading text="Videos" />

      {isAnyVideo ? (
        <ul className={styles.videos}>{getVideos()}</ul>
      ) : (
        <p>There aren&apos;t any videos at the moment.</p>
      )}
    </>
  ) : (
    <Loader />
  );
}

export { Videos };

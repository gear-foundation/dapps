import { Heading, Loader } from '@/components';
import { useParams } from 'react-router-dom';
import { ADDRESS } from '@/consts';
import { useSubscription } from '@/hooks';
import styles from './Video.module.scss';

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

function Video() {
  const { cid } = useParams();
  const isReady = useSubscription();

  const { name } = CIDs.find(({ id }) => id === cid) || {};

  return isReady ? (
    <div className={styles.wrapper}>
      <Heading text={name!} />

      <div className={styles.videoWrapper}>
        {/* eslint-disable-next-line jsx-a11y/media-has-caption */}
        <video controls>
          <source src={`${ADDRESS.IPFS_GATEWAY}${cid}`} type="video/mp4" />
        </video>
      </div>

      <p>{description}</p>
    </div>
  ) : (
    <Loader />
  );
}

export { Video };

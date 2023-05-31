import { Link } from 'react-router-dom';
import useEmblaCarousel from 'embla-carousel-react';
import clsx from 'clsx';
import { getIpfsAddress } from 'utils';
import { Container } from 'components';
import { ReactComponent as ArrowLeftSVG } from '../../assets/arrow-left.svg';
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

type Props = {
  slider?: boolean;
};

function NFTs({ slider }: Props) {
  const [emblaRef, emblaApi] = useEmblaCarousel({ align: 'center', loop: true });

  const prevSlide = () => emblaApi?.scrollPrev();
  const nextSlide = () => emblaApi?.scrollNext();

  const getNFTs = () =>
    LIST.map(({ id, collection, name, owner, media }) => {
      const style = { backgroundImage: `url(${getIpfsAddress(media)})` };
      const to = `/nft/${id}`;

      return (
        <li key={id} className={clsx(styles.nft, slider && styles.emblaSlide)}>
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
    </div>
  );
}

export { NFTs };

import { Button } from '@gear-js/vara-ui';

import { BannerLock, BannerSuits } from '@/assets/images';

import styles from './banner.module.scss';

type Props = {
  title: string;
  subtitle: string;
};

const Banner = ({ title, subtitle }: Props) => {
  return (
    <div className={styles.banner}>
      <div className={styles.image}>
        <BannerLock className={styles.center} />
        <img src={BannerSuits} alt="banner-suits" className={styles.center} />
      </div>

      <div className={styles.content}>
        <span className={styles.title}>{title}</span>
        <span className={styles.subtitle}>{subtitle}</span>
      </div>
      <Button className={styles.button} color="transparent">
        Learn more
      </Button>
    </div>
  );
};

export { Banner };

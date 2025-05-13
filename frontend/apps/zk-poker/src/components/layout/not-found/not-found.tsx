import clsx from 'clsx';
import { Link } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { buttonStyles } from '@/components';
import { Heading } from '@/components/ui/heading';

import ImageBase from './assets/images/404.jpg';
import ImageWebp from './assets/images/404.webp';
import styles from './not-found.module.scss';

export function NotFound() {
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.image}>
          <picture>
            <source srcSet={ImageWebp} type="image/webp" />
            <source srcSet={ImageBase} type="image/jpeg" />
            <img width={1668} height={934} src={ImageBase} alt="" loading="lazy" />
          </picture>
        </div>
        <div className={styles.header}>
          <Heading size="lg">Page not found</Heading>
        </div>
        <Link to={ROUTES.HOME} className={clsx(buttonStyles.button, buttonStyles.border)}>
          Back To Home
        </Link>
      </div>
    </div>
  );
}

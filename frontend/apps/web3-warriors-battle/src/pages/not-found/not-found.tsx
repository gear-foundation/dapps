import { buttonStyles } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { Link } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { Heading } from '@/components/ui/heading';

import imageSrc from './404.webp';
import styles from './not-found.module.scss';

export function NotFound() {
  return (
    <div className={styles.container}>
      <img src={imageSrc} alt="Not found" />

      <Heading size="lg">Page not found</Heading>

      <Link to={ROUTES.HOME} className={clsx(buttonStyles.button, buttonStyles.border, buttonStyles.default)}>
        Back To Home
      </Link>
    </div>
  );
}

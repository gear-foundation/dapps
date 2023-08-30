import { buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { Loader } from 'components';
import { useSubscription } from 'hooks';
import { Link } from 'react-router-dom';
import styles from './Home.module.scss';

function Home() {
  const isReady = useSubscription();

  return isReady ? (
    <>
      <Link
        to="subscription"
        className={clsx(buttonStyles.button, buttonStyles.large, buttonStyles.primary, styles.link)}>
        My Subscription
      </Link>
      <Link to="videos" className={clsx(buttonStyles.button, buttonStyles.large, buttonStyles.secondary)}>
        Videos
      </Link>
    </>
  ) : (
    <Loader />
  );
}

export { Home };

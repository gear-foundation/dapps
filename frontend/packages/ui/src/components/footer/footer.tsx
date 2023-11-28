import { useAccount } from '@gear-js/react-hooks';
import cx from 'clsx';

import { ReactComponent as UserSVG } from './assets/user.svg';
import { SOCIALS } from './consts';
import styles from './footer.module.css';

type Props = {
  vara?: boolean;
};

function Footer({ vara }: Props) {
  const { account } = useAccount();
  const year = new Date().getFullYear();

  const getSocials = () =>
    SOCIALS.map(({ href, SVG }) => (
      <li key={href}>
        <a href={href} target="_blank" rel="noreferrer">
          <SVG />
        </a>
      </li>
    ));

  return (
    <footer className={cx(styles.footer, vara && styles.vara)}>
      {/* TODO: should be wrapped in a container? */}
      <div className={styles.column}>
        <ul className={styles.socials}>{getSocials()}</ul>

        <small className={cx(styles.copyright, vara && styles.vara)}>
          &copy; {year} Gear Foundation, Inc. All Rights Reserved.
        </small>
      </div>

      {account && (
        <a
          href={`https://vara.subscan.io/account/${account.address}`}
          target="_blank"
          rel="noreferrer"
          className={cx(styles.explorerLink, vara && styles.vara)}>
          <UserSVG />
          <span>View in Blockchain Explorer</span>
        </a>
      )}
    </footer>
  );
}

export { Footer };

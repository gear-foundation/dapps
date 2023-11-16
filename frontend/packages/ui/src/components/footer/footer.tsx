import { useAccount } from '@gear-js/react-hooks';
import cx from 'clsx';

import { Props } from '../../types';
import { ReactComponent as TwitterSVG } from './assets/twitter.svg';
import { ReactComponent as GithubSVG } from './assets/github.svg';
import { ReactComponent as DiscordSVG } from './assets/discord.svg';
import { ReactComponent as MediumSVG } from './assets/medium.svg';
import { ReactComponent as UserSVG } from './assets/user.svg';
import styles from './footer.module.css';

const SOCIALS = [
  { href: 'https://twitter.com/gear_techs', SVG: TwitterSVG },
  { href: 'https://github.com/gear-tech', SVG: GithubSVG },
  { href: 'https://discord.com/invite/7BQznC9uD9', SVG: DiscordSVG },
  { href: 'https://medium.com/@gear_techs', SVG: MediumSVG },
];

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

import DiscordSVG from '@/assets/images/socials/discord.svg?react';
import GithubSVG from '@/assets/images/socials/github.svg?react';
import MediumSVG from '@/assets/images/socials/medium.svg?react';
import TwitterSVG from '@/assets/images/socials/twitter.svg?react';

import styles from './Socials.module.scss';

const socials = [
  { href: 'https://twitter.com/VaraNetwork', icon: TwitterSVG },
  { href: 'https://github.com/gear-foundation', icon: GithubSVG },
  { href: 'https://discord.gg/x8ZeSy6S6K', icon: DiscordSVG },
  { href: 'https://medium.com/@VaraNetwork', icon: MediumSVG },
];

function Socials() {
  const getItems = () =>
    socials.map(({ href, icon: Icon }) => (
      <li key={href}>
        <a href={href} target="_blank" rel="noreferrer">
          <Icon />
        </a>
      </li>
    ));

  return <ul className={styles.socials}>{getItems()}</ul>;
}

export { Socials };

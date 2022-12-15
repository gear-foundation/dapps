import { DiscordSVG, GithubSVG, TwitterSVG, MediumSVG } from 'components/layout/icons';
import styles from './Socials.module.scss';

const socials = [
  { href: 'https://twitter.com/gear_techs', icon: TwitterSVG },
  { href: 'https://github.com/gear-tech', icon: GithubSVG },
  { href: 'https://discord.com/invite/7BQznC9uD9', icon: DiscordSVG },
  { href: 'https://medium.com/@gear_techs', icon: MediumSVG },
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

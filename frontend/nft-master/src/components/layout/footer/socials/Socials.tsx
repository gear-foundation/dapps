import { DiscordIcon, GithubIcon, MediumIcon, TwitterIcon } from 'assets/images';
import styles from './Socials.module.scss';

const socials = [
  { href: 'https://twitter.com/VaraNetwork', icon: TwitterIcon },
  { href: 'https://github.com/gear-tech', icon: GithubIcon },
  { href: 'https://discord.com/invite/7BQznC9uD9', icon: DiscordIcon },
  { href: 'https://medium.com/@VaraNetwork', icon: MediumIcon },
];

export function Socials() {
  return (
    <ul className={styles.socials}>
      {socials.map(({ href, icon: Icon }) => (
        <li key={href}>
          <a className={styles.link} href={href} target="_blank" rel="noreferrer">
            <Icon />
          </a>
        </li>
      ))}
    </ul>
  );
}

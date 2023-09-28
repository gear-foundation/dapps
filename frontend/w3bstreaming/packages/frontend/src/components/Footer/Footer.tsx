import { cx } from '@/utils';
import styles from './Footer.module.scss';
import logo from '@/assets/icons/logo.png';
import web from '@/assets/icons/web-icon.png';
import discord from '@/assets/icons/discord-icon.png';
import git from '@/assets/icons/git-icon.png';
import twitter from '@/assets/icons/twitter-icon.png';

const socials = [
  {
    name: 'web',
    url: 'https://medium.com/@VaraNetwork',
    icon: web,
  },
  {
    name: 'discord',
    url: 'https://discord.gg/x8ZeSy6S6K',
    icon: discord,
  },
  {
    name: 'git',
    url: 'https://github.com/gear-foundation',
    icon: git,
  },
  {
    name: 'twitter',
    url: 'https://twitter.com/VaraNetwork',
    icon: twitter,
  },
];

function Footer() {
  return (
    <footer className={cx(styles.footer)}>
      <div className={cx(styles.container)}>
        <img src={logo} alt="" />
        <span className={cx(styles.rigts)}>© 2023 Gear Foundation, Inc. All Rights Reserved.</span>
        <div className={cx(styles.socials)}>
          {socials.map(({ name, url, icon }) => (
            <a href={url} key={`${name}${url}`}>
              <img src={icon} alt={name} className={cx(styles['socials-icon'])} />
            </a>
          ))}
        </div>
      </div>
    </footer>
  );
}

export { Footer };

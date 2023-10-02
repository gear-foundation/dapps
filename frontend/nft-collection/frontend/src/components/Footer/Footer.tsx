import { cx } from '@/utils';
import styles from './Footer.module.scss';
import logo from '@/assets/icons/logo-vara-black.svg';
import web from '@/assets/icons/web-icon.svg';
import discord from '@/assets/icons/discord-icon.svg';
import git from '@/assets/icons/git-icon.svg';
import twitter from '@/assets/icons/twitter-icon.svg';

const socials = [
  {
    name: 'twitter',
    url: 'https://twitter.com/gear_techs',
    icon: twitter,
  },
  {
    name: 'git',
    url: 'https://github.com/gear-tech',
    icon: git,
  },
  {
    name: 'discord',
    url: 'https://discord.com/invite/7BQznC9uD9',
    icon: discord,
  },
  {
    name: 'web',
    url: 'https://medium.com/@gear_techs',
    icon: web,
  },
];

function Footer() {
  return (
    <footer className={cx(styles.footer)}>
      <div className={cx(styles.container)}>
        <img src={logo} alt="" className={cx(styles.logo)} />
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

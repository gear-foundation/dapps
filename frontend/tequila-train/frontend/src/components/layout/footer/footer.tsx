import { Icon } from 'components/ui/icon';

const socials = [
  { href: 'https://twitter.com/gear_techs', icon: 'twitter' },
  { href: 'https://github.com/gear-tech', icon: 'github' },
  { href: 'https://discord.com/invite/7BQznC9uD9', icon: 'discord' },
  { href: 'https://medium.com/@gear_techs', icon: 'medium' },
];

export const Footer = () => (
  <footer className="container flex items-center gap-4 justify-between py-4">
    <ul className="flex gap-5">
      {socials.map(({ href, icon }) => (
        <li key={icon}>
          <a
            href={href}
            target="_blank"
            rel="noreferrer"
            className="text-dark-500 text-opacity-50 hover:text-opacity-80 transition-colors">
            <Icon name={icon} className="w-6 h-6" />
          </a>
        </li>
      ))}
    </ul>
    <p className="text-dark-500 text-opacity-50 text-xs">
      &copy; {new Date().getFullYear()} Gear Foundation, Inc. All Rights Reserved.
    </p>
  </footer>
);

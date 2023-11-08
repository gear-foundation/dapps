import clsx from 'clsx';
import styles from './Socials.module.scss';
import { BaseComponentProps } from '@/app/types';
import { Sprite } from '@/components/ui/sprite';

const socials = [
  { href: 'https://twitter.com/gear_techs', icon: 'twitter' },
  { href: 'https://github.com/gear-tech', icon: 'github' },
  { href: 'https://discord.com/invite/7BQznC9uD9', icon: 'discord' },
  { href: 'https://medium.com/@gear_techs', icon: 'medium' },
];

export function Socials({ className }: BaseComponentProps) {
  return (
    <ul className={clsx(className, styles.socials)}>
      {socials.map(({ href, icon }) => (
        <li key={href}>
          <a href={href} target="_blank" rel="noreferrer">
            <Sprite name={icon} size={31} />
          </a>
        </li>
      ))}
    </ul>
  );
}

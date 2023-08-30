import clsx from 'clsx'
import styles from './Socials.module.scss'
import {
  DiscordIcon,
  GithubIcon,
  MediumIcon,
  TwitterIcon,
} from '@/assets/images'

const socials = [
  { href: 'https://twitter.com/gear_techs', icon: TwitterIcon },
  { href: 'https://github.com/gear-tech', icon: GithubIcon },
  { href: 'https://discord.com/invite/7BQznC9uD9', icon: DiscordIcon },
  { href: 'https://medium.com/@gear_techs', icon: MediumIcon },
]

export function Socials({ className }: BaseComponentProps) {
  return (
    <ul className={clsx(className, styles.socials)}>
      {socials.map(({ href, icon: Icon }) => (
        <li key={href}>
          <a href={href} target="_blank" rel="noreferrer">
            <Icon />
          </a>
        </li>
      ))}
    </ul>
  )
}

import { NavLink } from 'react-router-dom'
import clsx from 'clsx'
import styles from './navigation.module.scss'
import { useAuth } from '@/features/auth'
import { ROUTES } from '@/app/consts'

const nav = [
  {
    id: 'home',
    url: ROUTES.HOME,
    label: 'Play',
    isPrivate: true,
  },
  {
    id: 'leaderboard',
    url: ROUTES.LEADERBOARD,
    label: 'Leaderboard',
    isPrivate: false,
  },
  // {
  //   id: 'notfound',
  //   url: ROUTES.NOTFOUND,
  //   label: '404',
  // },
]

export function Navigation() {
  const { authToken } = useAuth()
  return (
    <div>
      <nav>
        <ul className={styles.list}>
          {nav.map(({ id, url, label, isPrivate }) => (
            <li key={id}>
              <NavLink
                to={url}
                className={({ isActive }) =>
                  clsx(styles.link, isActive ? styles.active : styles.base)
                }
                aria-disabled={isPrivate && !authToken}
                end
              >
                {label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
    </div>
  )
}

import { NavLink } from 'react-router-dom';
import clsx from 'clsx';
import styles from './navigation.module.scss';
import { ROUTES } from '@/app/consts';

const nav = [
  {
    id: 'home',
    url: ROUTES.HOME,
    label: 'Play',
    isPrivate: true,
  },
];

export function Navigation() {
  return (
    <div>
      <nav>
        <ul className={styles.list}>
          {nav.map(({ id, url, label, isPrivate }) => (
            <li key={id}>
              <NavLink
                to={url}
                className={({ isActive }) => clsx(styles.link, isActive ? styles.active : styles.base)}
                aria-disabled={isPrivate}
                end>
                {label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
    </div>
  );
}

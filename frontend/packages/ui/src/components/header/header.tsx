import { clsx } from 'clsx';
import styles from './header.module.css';
import { PropsWithChildren } from 'react';

type Props = {
  logo: JSX.Element;
  className?: {
    header?: string;
    content?: string;
  };
  menu: JSX.Element;
} & PropsWithChildren;

function Header({ logo, className, menu, children }: Props) {
  return (
    <header className={clsx(styles.header, className?.header)}>
      <div className={clsx(styles.content, className?.content)}>
        {logo}
        {children}
        <>{menu}</>
      </div>
    </header>
  );
}

export { Header };

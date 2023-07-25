import { Logo } from './logo'
import styles from './header.module.scss'
import { Navigation } from './navigation'
import { AccountInfo } from './account-info'
import { Container } from '@/components/ui/container'

export function Header() {
  return (
    <header className={styles.header}>
      <Container className={styles.header__container}>
        <Logo className={styles.header__logo} />
        <Navigation />
        <AccountInfo />
      </Container>
    </header>
  )
}

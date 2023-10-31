import { Logo } from './logo'
import styles from './header.module.scss'
import { Container } from '@/components/ui/container'
import clsx from 'clsx'
import { useAccount } from '@gear-js/react-hooks'
import { MobileMenu } from '@/components/layout/header/mobile-menu'
import { Wallet } from '@/features/wallet'

export function Header() {
  const { account } = useAccount()
  return (
    <header className={styles.header}>
      <Container className={styles.header__container}>
        <Logo
          className={clsx(
            styles.header__logo,
            !account && styles['header__logo--center']
          )}
          label="Tic-Tac-Toe"
        />
        {!!account && <MobileMenu />}
        <div className={styles.header__wallet}>
          <Wallet className={styles.wallet} />
        </div>
      </Container>
    </header>
  )
}

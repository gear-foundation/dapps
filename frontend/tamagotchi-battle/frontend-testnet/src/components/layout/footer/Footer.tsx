import { Socials } from './socials'
import { Copyright } from './copyright'
import { Container } from '@/components/ui/container'
import styles from './Footer.module.scss'

function Footer() {
  return (
    <footer className={styles.footer}>
      <Container className={styles.footer__container}>
        <Socials className={styles.footer__socials} />
        <Copyright />
      </Container>
    </footer>
  )
}

export { Footer }

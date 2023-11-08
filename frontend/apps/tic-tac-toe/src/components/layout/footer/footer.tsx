import styles from './footer.module.scss';
import { Socials } from './socials';
import { Copyright } from './copyright';
import { Container } from '@/components/ui/container';
import { useGame } from '@/features/tic-tac-toe/hooks';
import { AnimatePresence, motion, Variants } from 'framer-motion';
import clsx from 'clsx';
import { Sprite } from '@/components/ui/sprite';

export const variantsBanner: Variants = {
  enter: { opacity: 0 },
  center: {
    opacity: 1,
    transition: {
      ease: 'easeOut',
      duration: 0.3,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      ease: 'easeIn',
      duration: 0.2,
    },
  },
};

export function Footer() {
  const { gameState } = useGame();

  return (
    <footer className={styles.footer}>
      <AnimatePresence>
        {!!gameState?.gameOver && (
          <motion.section
            initial="enter"
            animate="center"
            exit="exit"
            variants={variantsBanner}
            className={styles.banner}>
            <Container>
              <div className={styles.banner__backdrop} />
              <div className={styles.banner__container}>
                <div className={styles.banner__right}>
                  <h2 className={styles.banner__title}>
                    Thank you for your interest <span>in the Vara Network.</span>
                  </h2>
                  <div className={styles.banner__text}>
                    <p>You've experienced a fully on-chain game.</p>
                    <p>
                      We look forward to having you join the ranks of developers shaping the new generation of
                      decentralized applications.
                    </p>
                  </div>
                </div>
                <ul className={styles.banner__left}>
                  <li className={styles.banner__item}>
                    <div className={styles.banner__icon}>
                      <Sprite name="gear-logo" size={24} />
                    </div>
                    <p className="">
                      Visit the{' '}
                      <a href="https://wiki.gear-tech.io/" target="_blank" rel="noreferrer">
                        Gear Wiki
                      </a>{' '}
                      to discover how easy it is to create programs using the Gear Protocol.
                    </p>
                  </li>
                  <li className={styles.banner__item}>
                    <div className={styles.banner__icon}>
                      <Sprite name="gear-logo" size={24} />
                    </div>
                    <p className="">
                      Consider enrolling in a free course at{' '}
                      <a href="https://academy.gear.foundation/" target="_blank" rel="noreferrer">
                        Gear&nbsp;Academy
                      </a>{' '}
                      to become a top-notch Web3 developer.
                    </p>
                  </li>
                </ul>
              </div>
            </Container>
          </motion.section>
        )}
      </AnimatePresence>

      <Container className={clsx(styles.footer__container, !!gameState?.gameOver && styles.mobile)}>
        <Sprite name="vara-logo" className={styles.footer__logo} aria-label="Vara Network logotype" />

        <Copyright className={styles.footer__copyright} />

        <Socials className={styles.footer__socials} />
      </Container>
    </footer>
  );
}

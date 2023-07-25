import styles from './leaderboard.module.scss'
import { Heading } from '@/components/ui/heading'
import { useAccount } from '@gear-js/react-hooks'
import { Wallet } from '@/features/wallet'
import { Container } from '@/components/ui/container'
import { Link } from 'react-router-dom'
import { ROUTES } from '@/app/consts'
import { buttonVariants } from '@/components/ui/button/button'
import { Leaderboard } from '@/features/tic-tac-toe/components/leaderboard'
import { Text } from '@/components/ui/text'
import { TextGradient } from '@/components/ui/text-gradient'

export default function PageLeaderboard() {
  const { account } = useAccount()
  return (
    <section className={styles.section}>
      <Container className={styles.container}>
        <div className={styles.header}>
          <Heading>
            <TextGradient>Tic Tac Toe game</TextGradient>
          </Heading>

          <div className={styles.subheadings}>
            {account ? (
              <Text size="lg">
                A classic game of tic-tac-toe in which you compete not against a
                human, but against a smart contract. Play to win PPV.
              </Text>
            ) : (
              <>
                <Text size="lg">
                  To register, follow the referral link provided by your friend
                  or Vara Network community manager.
                </Text>
                <Text size="lg">
                  If you are already registered connect your Substrate wallet to
                  continue.
                </Text>
              </>
            )}
          </div>

          {account ? (
            <Link to={ROUTES.HOME} className={buttonVariants()}>
              Play
            </Link>
          ) : (
            <Wallet account={account} isReady />
          )}
        </div>

        <div className={styles.content}>
          <Leaderboard />
        </div>
      </Container>
    </section>
  )
}

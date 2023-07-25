import WelcomePNG from '../../assets/images/welcome-bg.png'
import WelcomeWebp from '../../assets/images/welcome-bg.webp'
import { ColumnLeft, ColumnRight, ColumnsContainer } from '../ui/columns'
import { HelpDescription } from '../ui/typography'
import styles from './welcome.module.scss'
import { Heading } from '@/components/ui/heading'
import { TextGradient } from '@/components/ui/text-gradient'

export function Welcome({ children }: React.PropsWithChildren) {
  return (
    <ColumnsContainer>
      <ColumnLeft>
        <Heading>
          <TextGradient>Tic Tac Toe game</TextGradient>
        </Heading>
        <HelpDescription>
          <p>
            A classic game of tic-tac-toe in which you compete not against a
            human, but against a smart contract. Play to win PPV.
          </p>
        </HelpDescription>
        <div>{children}</div>
      </ColumnLeft>
      <ColumnRight>
        <div className={styles.image}>
          <picture>
            <source srcSet={WelcomeWebp} type="image/webp" />
            <source srcSet={WelcomePNG} type="image/png" />
            <img
              width={470}
              height={428}
              src={WelcomePNG}
              alt="Welcome!"
              loading="lazy"
            />
          </picture>
        </div>
      </ColumnRight>
    </ColumnsContainer>
  )
}

import WelcomePNG from '../../assets/images/welcome-bg.png';
import WelcomeWebp from '../../assets/images/welcome-bg.webp';
import { ColumnLeft, ColumnRight, ColumnsContainer } from '../ui/columns';
import { HelpDescription } from '../ui/typography';
import styles from './welcome.module.scss';
import { Heading } from '@/components/ui/heading';
import { TextGradient } from '@/components/ui/text-gradient';
import { BaseComponentProps } from '@/app/types';

export function Welcome({ children,  }: BaseComponentProps) {
  return (
    <ColumnsContainer className={styles.wrapper}>
      <ColumnLeft>
        <Heading className={styles.heading}>
          <TextGradient>Tic Tac Toe game</TextGradient>
        </Heading>
        <HelpDescription className={styles.text}>
          <p>A classic game of tic-tac-toe in which you compete not against a human, but against a smart contract.</p>
        </HelpDescription>
        <div className={styles.content}>{children}</div>
      </ColumnLeft>
      <ColumnRight>
        <div className={styles.image}>
          <picture>
            <source srcSet={WelcomeWebp} type="image/webp" />
            <source srcSet={WelcomePNG} type="image/png" />
            <img width={470} height={428} src={WelcomePNG} alt="Welcome!" loading="lazy" />
          </picture>
        </div>
      </ColumnRight>
    </ColumnsContainer>
  );
}

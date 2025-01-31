import { Button } from '@gear-js/ui';
import { ReactComponent as NextSVG } from '@/assets/images/buttons/next.svg';
import { ReactComponent as LastSVG } from '@/assets/images/buttons/last.svg';
import styles from './Buttons.module.scss';

type Props = {
  onFirstClick?: () => void;
  onPrevClick?: () => void;
  onNextClick?: () => void;
  onLastClick?: () => void;
  isPauseButton?: boolean;
};

function Buttons({ onFirstClick, onPrevClick, onNextClick, onLastClick }: Props) {
  return (
    <div>
      {onFirstClick && (
        <Button icon={LastSVG} color="transparent" className={styles.backButton} onClick={onFirstClick} />
      )}

      <div className={styles.mainButtons}>
        {onPrevClick && (
          <Button icon={NextSVG} color="transparent" className={styles.backButton} onClick={onPrevClick} />
        )}
        {onNextClick && <Button icon={NextSVG} color="transparent" onClick={onNextClick} />}
      </div>

      {onLastClick && <Button icon={LastSVG} color="transparent" onClick={onLastClick} />}
    </div>
  );
}

export { Buttons };

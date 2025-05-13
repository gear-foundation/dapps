import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';

import { AllInIcon, CallIcon, Chips2xIcon, Chips3xIcon, Chips5xIcon, FoldIcon } from '@/assets/images';

import styles from './game-buttons.module.scss';

type Props = {
  className?: string;
  disabled?: boolean;
};

const GameButtons = ({ className, disabled = false }: Props) => {
  const handleAllIn = () => {
    console.log('All In');
  };

  const handleCall = () => {
    console.log('Call');
  };

  const handleFold = () => {
    console.log('Fold');
  };

  const handleRaise = (multiplier: number) => {
    console.log('Raise', multiplier);
  };

  return (
    <>
      <div className={clsx(styles.gameButtons, className)}>
        <div className={styles.allInContainer}>
          <Button onClick={handleAllIn} disabled={disabled} color="transparent">
            <AllInIcon />
          </Button>
        </div>
        <div className={styles.actionButtons}>
          <Button onClick={handleFold} disabled={disabled} color="transparent">
            <FoldIcon />
          </Button>
          <Button onClick={handleCall} disabled={disabled} color="transparent">
            <CallIcon />
          </Button>
          <Button onClick={() => handleRaise(2)} disabled={disabled} color="transparent">
            <Chips2xIcon />
          </Button>
          <Button onClick={() => handleRaise(3)} disabled={disabled} color="transparent">
            <Chips3xIcon />
          </Button>
          <Button onClick={() => handleRaise(5)} disabled={disabled} color="transparent">
            <Chips5xIcon />
          </Button>
        </div>
      </div>
    </>
  );
};

export { GameButtons };

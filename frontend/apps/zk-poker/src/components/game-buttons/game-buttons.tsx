import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';

import { AllInIcon, CallIcon, Chips2xIcon, Chips3xIcon, Chips5xIcon, FoldIcon } from '@/assets/images';
import { useTurnMessage } from '@/features/game/sails';

import styles from './game-buttons.module.scss';

type Props = {
  className?: string;
  disabled?: boolean;
  currentBet: number;
  myCurrentBet: number;
  bigBlind: number;
};

const GameButtons = ({ className, disabled = false, currentBet, myCurrentBet, bigBlind }: Props) => {
  const { turnMessage } = useTurnMessage();

  const handleAllIn = () => {
    void turnMessage({ action: { allIn: null } });
  };

  const handleCall = () => {
    if (myCurrentBet === currentBet || currentBet === 0) {
      void turnMessage({ action: { check: null } });
    } else {
      void turnMessage({ action: { call: null } });
    }
  };

  const handleCheck = () => {
    void turnMessage({ action: { check: null } });
  };

  const handleFold = () => {
    void turnMessage({ action: { fold: null } });
  };

  const handleRaise = (multiplier: number) => {
    void turnMessage({ action: { raise: { bet: multiplier * (currentBet || bigBlind) } } });
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
          {myCurrentBet === currentBet || currentBet === 0 ? (
            <Button onClick={handleCheck} disabled={disabled} color="transparent">
              {/* // ! TODO: add check icon*/}
              <CallIcon />
            </Button>
          ) : (
            <Button onClick={handleCall} disabled={disabled} color="transparent">
              <CallIcon />
            </Button>
          )}
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

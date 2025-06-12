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
  const { turnMessage, isPending } = useTurnMessage();

  const handleAllIn = () => {
    void turnMessage({ action: { allIn: null } });
  };

  const handleCall = () => {
    void turnMessage({ action: { call: null } });
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

  const isDisabled = disabled || isPending;

  return (
    <>
      <div className={clsx(styles.gameButtons, className)}>
        <div className={styles.allInContainer}>
          <Button onClick={handleAllIn} disabled={isDisabled} color="transparent">
            <AllInIcon />
          </Button>
        </div>
        <div className={styles.actionButtons}>
          <Button onClick={handleFold} disabled={isDisabled} color="transparent">
            <FoldIcon />
          </Button>

          {myCurrentBet === currentBet || currentBet === 0 ? (
            <Button onClick={handleCheck} disabled={isDisabled} color="transparent">
              {/* // ! TODO: add check icon*/}
              <CallIcon />
            </Button>
          ) : (
            <Button onClick={handleCall} disabled={isDisabled} color="transparent">
              <CallIcon />
            </Button>
          )}

          <Button onClick={() => handleRaise(2)} disabled={isDisabled} color="transparent">
            <Chips2xIcon />
          </Button>
          <Button onClick={() => handleRaise(3)} disabled={isDisabled} color="transparent">
            <Chips3xIcon />
          </Button>
          <Button onClick={() => handleRaise(5)} disabled={isDisabled} color="transparent">
            <Chips5xIcon />
          </Button>
        </div>
      </div>
    </>
  );
};

export { GameButtons };

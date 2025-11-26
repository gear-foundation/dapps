import { Button } from '@gear-js/vara-ui';
import * as Slider from '@radix-ui/react-slider';
import clsx from 'clsx';
import { useState } from 'react';

import {
  AllInIcon,
  ManualIcon,
  CallIcon,
  Chips2xIcon,
  Chips3xIcon,
  Chips5xIcon,
  FoldIcon,
  CancelIcon,
} from '@/assets/images';
import { useTurnMessage } from '@/features/game/sails';

import styles from './game-buttons.module.scss';

type Props = {
  className?: string;
  disabled?: boolean;
  currentBet: number;
  myCurrentBet: number;
  bigBlind: number;
  balance: number;
};

const GameButtons = ({ className, disabled = false, currentBet, myCurrentBet, bigBlind, balance }: Props) => {
  const { turnMessage, isPending: isTurnMessagePending } = useTurnMessage();
  const [isSliderActive, setIsSliderActive] = useState(false);
  const [sliderValue, setSliderValue] = useState(0);

  const minBet = Math.max(currentBet - myCurrentBet, bigBlind);
  const maxBet = balance;

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
    void turnMessage({ action: { raise: { bet: multiplier * (currentBet || bigBlind) - myCurrentBet } } });
  };

  const handleToggleSlider = () => {
    if (!isSliderActive) {
      setSliderValue(minBet);
    }
    setIsSliderActive(!isSliderActive);
  };

  const handleSliderConfirm = () => {
    const callAmount = currentBet - myCurrentBet;

    if (sliderValue === balance) {
      void turnMessage({ action: { allIn: null } });
    } else if (sliderValue === callAmount && callAmount > 0) {
      void turnMessage({ action: { call: null } });
    } else {
      void turnMessage({ action: { raise: { bet: sliderValue } } });
    }
    setIsSliderActive(false);
  };

  const getHasEnoughBalance = (multiplier: number) => {
    return balance >= multiplier * (currentBet || bigBlind);
  };

  const isDisabled = disabled;
  const isCallDisabled = balance <= currentBet - myCurrentBet;
  const isCheck = myCurrentBet === currentBet || currentBet === 0;
  const isManualDisabled = balance <= minBet;

  const raise2Amount = 2 * (currentBet || bigBlind) - myCurrentBet;
  const raise3Amount = 3 * (currentBet || bigBlind) - myCurrentBet;
  const raise5Amount = 5 * (currentBet || bigBlind) - myCurrentBet;
  const callAmount = currentBet - myCurrentBet;

  return isTurnMessagePending ? null : (
    <>
      <div className={clsx(styles.gameButtons, className)}>
        <div className={styles.topButtons}>
          <Button onClick={handleFold} disabled={isDisabled} color="transparent">
            <FoldIcon />
          </Button>
          <Button onClick={handleAllIn} disabled={isDisabled} color="transparent">
            <AllInIcon />
          </Button>
        </div>

        <div className={styles.actionButtons}>
          {!isSliderActive ? (
            <>
              <Button
                onClick={handleToggleSlider}
                disabled={isDisabled || isManualDisabled}
                color="transparent"
                className={styles.button}>
                <ManualIcon />
                <span className={styles.text}>Manual</span>
              </Button>

              <Button
                onClick={() => handleRaise(2)}
                disabled={isDisabled || !getHasEnoughBalance(2)}
                color="transparent"
                className={styles.button}>
                <Chips2xIcon />
                {getHasEnoughBalance(2) && <span className={styles.raiseText}>{raise2Amount}</span>}
              </Button>
              <Button
                onClick={() => handleRaise(3)}
                disabled={isDisabled || !getHasEnoughBalance(3)}
                color="transparent"
                className={styles.button}>
                <Chips3xIcon />
                {getHasEnoughBalance(3) && <span className={styles.raiseText}>{raise3Amount}</span>}
              </Button>
              <Button
                onClick={() => handleRaise(5)}
                disabled={isDisabled || !getHasEnoughBalance(5)}
                color="transparent"
                className={styles.button}>
                <Chips5xIcon />
                {getHasEnoughBalance(5) && <span className={styles.raiseText}>{raise5Amount}</span>}
              </Button>

              <Button
                onClick={isCheck ? handleCheck : handleCall}
                disabled={isDisabled || isCallDisabled}
                color="transparent"
                className={styles.button}>
                <CallIcon />
                <span className={styles.text}>{isCheck ? 'Check' : `Call`}</span>
                {!isCallDisabled && !isCheck && <span className={styles.raiseText}>{callAmount}</span>}
              </Button>
            </>
          ) : (
            <>
              <Button onClick={handleToggleSlider} disabled={isDisabled} color="transparent" className={styles.button}>
                <CancelIcon />
                <span className={clsx(styles.text, styles.cancel)}>Cancel</span>
              </Button>
              <div className={styles.sliderContainer}>
                <div className={styles.sliderControls}>
                  <span className={styles.sliderValue}>{sliderValue}</span>
                </div>
                <Slider.Root
                  className={styles.sliderRoot}
                  value={[sliderValue]}
                  onValueChange={(value) => setSliderValue(value[0])}
                  min={minBet}
                  max={maxBet}
                  step={bigBlind}
                  disabled={isDisabled}>
                  <Slider.Track className={styles.sliderTrack}>
                    <Slider.Range className={styles.sliderRange} />
                  </Slider.Track>
                  <Slider.Thumb className={styles.sliderThumb} />
                </Slider.Root>
              </div>
              <Button onClick={handleSliderConfirm} disabled={isDisabled} color="transparent" className={styles.button}>
                <CallIcon />
                <span className={styles.text}>Bet</span>
              </Button>
            </>
          )}
        </div>
      </div>
    </>
  );
};

export { GameButtons };

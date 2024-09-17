import React, { useRef, useState, useEffect, useCallback } from 'react';
import { GameEngine } from '../../models/Game';
import { useMediaQuery } from '@/hooks/use-mobile-device';
import { MOBILE_BREAKPOINT } from '@/app/consts';
import MoveControlImg from '../../assets/images/move-control.png';
import MoveShiftImg from '../../assets/images/move-shift.png';

interface MobileControllerProps {
  gameInstanceRef: React.RefObject<GameEngine | null>;
}

const MobileController = ({ gameInstanceRef }: MobileControllerProps) => {
  const isMobile = useMediaQuery(MOBILE_BREAKPOINT);
  const circleRef = useRef<HTMLDivElement | null>(null);
  const shiftRef = useRef<HTMLDivElement | null>(null);
  const [isTouching, setIsTouching] = useState(false);
  const [isShift, setIsShift] = useState(false);
  const [movementAngle, setMovementAngle] = useState<number | null>(null);
  const [scrollUnlockTimer, setScrollUnlockTimer] = useState<NodeJS.Timeout | null>(null);

  const blockScroll = useCallback(() => {
    if (scrollUnlockTimer) {
      clearTimeout(scrollUnlockTimer);
      setScrollUnlockTimer(null);
    }
    document.body.style.overflow = 'hidden';
    document.addEventListener('touchmove', preventDefault, { passive: false });
  }, [scrollUnlockTimer]);

  const unblockScroll = useCallback(() => {
    document.body.style.overflow = '';
    document.removeEventListener('touchmove', preventDefault);
  }, []);

  const preventDefault = (e: TouchEvent) => {
    if (e.cancelable) {
      e.preventDefault();
    }
  };

  const handleShiftTouchStart = () => {
    if (isTouching && !isShift) {
      setIsShift(true);
      blockScroll();
    }
  };

  const handleShiftTouchEnd = () => {
    setIsShift(false);
    setMovementAngle(null);
    startScrollUnlockTimer();
  };

  const handleTouchStart = (event: React.TouchEvent) => {
    const touch = event.touches[0];
    if (touch && isInsideCircle(touch.clientX, touch.clientY, circleRef)) {
      setIsTouching(true);
      blockScroll();
      handleMovement(event);
    }
  };

  const handleTouchMove = (event: React.TouchEvent) => {
    if (isTouching) {
      handleMovement(event);
    }
  };

  const handleTouchEnd = () => {
    setIsTouching(false);
    setMovementAngle(null);
    startScrollUnlockTimer();
  };

  const handleMovement = (event: React.TouchEvent) => {
    if (!circleRef.current) return;
    const touch = event.touches[0];

    if (touch) {
      const circleRect = circleRef.current.getBoundingClientRect();
      const circleX = circleRect.left + circleRect.width / 2;
      const circleY = circleRect.top + circleRect.height / 2;
      const touchX = touch.clientX;
      const touchY = touch.clientY;
      const deltaX = touchX - circleX;
      const deltaY = touchY - circleY;

      const angle = Math.atan2(deltaY, deltaX) * (180 / Math.PI);
      setMovementAngle(angle);
    }
  };

  const isInsideCircle = (x: number, y: number, ref: React.RefObject<HTMLDivElement | null>) => {
    if (!ref.current) return false;
    const rect = ref.current.getBoundingClientRect();
    return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
  };

  useEffect(() => {
    let animationFrameId: number;

    const moveCharacter = () => {
      const character = gameInstanceRef.current?.getCharacter();
      if (character && movementAngle !== null) {
        character.updateMovementByAngle(movementAngle, isShift);
      }
      animationFrameId = requestAnimationFrame(moveCharacter);
    };

    if (isTouching) {
      animationFrameId = requestAnimationFrame(moveCharacter);
    }

    return () => cancelAnimationFrame(animationFrameId);
  }, [isTouching, movementAngle, gameInstanceRef, isShift]);

  useEffect(() => {
    const handleBeforeUnload = () => {
      unblockScroll();
    };

    const handleVisibilityChange = () => {
      if (document.visibilityState === 'hidden') {
        unblockScroll();
      }
    };

    window.addEventListener('beforeunload', handleBeforeUnload);
    document.addEventListener('visibilitychange', handleVisibilityChange);

    return () => {
      window.removeEventListener('beforeunload', handleBeforeUnload);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      unblockScroll();
    };
  }, [unblockScroll]);

  const startScrollUnlockTimer = () => {
    if (scrollUnlockTimer) {
      clearTimeout(scrollUnlockTimer);
    }
    const timer = setTimeout(() => {
      unblockScroll();
      setScrollUnlockTimer(null);
    }, 3000);
    setScrollUnlockTimer(timer);
  };

  return (
    <>
      {isMobile && (
        <div className="fixed left-0 bottom-10 w-full transform z-2 flex justify-around">
          <div ref={shiftRef} onTouchStart={handleShiftTouchStart} onTouchEnd={handleShiftTouchEnd}>
            <img src={MoveShiftImg} alt="" style={{ width: '100px', height: '100px' }} />
          </div>
          <div
            ref={circleRef}
            onTouchStart={handleTouchStart}
            onTouchMove={handleTouchMove}
            onTouchEnd={handleTouchEnd}>
            <img src={MoveControlImg} alt="" style={{ width: '100px', height: '100px' }} />
          </div>
        </div>
      )}
    </>
  );
};

export default MobileController;

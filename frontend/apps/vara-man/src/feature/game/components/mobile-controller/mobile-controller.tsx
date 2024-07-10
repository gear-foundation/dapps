import React, { useRef, useState, useEffect } from 'react';

import { GameEngine } from '../../models/Game';
import { useMediaQuery } from '@/hooks/use-mobile-device';

import MoveControlImg from '../../assets/images/move-control.png';
import MoveShiftImg from '../../assets/images/move-shift.png';

interface MobileControllerProps {
  gameInstanceRef: React.RefObject<GameEngine | null>;
}

const MobileController = ({ gameInstanceRef }: MobileControllerProps) => {
  const isMobile = useMediaQuery('(max-width: 768px)');
  const circleRef = useRef<HTMLDivElement | null>(null);
  const [isTouching, setIsTouching] = useState(false);
  const [isShift, setIsShift] = useState(false);
  const [movementAngle, setMovementAngle] = useState<number | null>(null);

  const handleShiftTouchStart = () => {
    if (isTouching && !isShift) {
      setIsShift(true);
      blockScroll();
    }
  };

  const handleShiftTouchEnd = () => {
    setIsShift(false);
    unblockScroll();
    setMovementAngle(null);
  };

  const handleTouchStart = (event: React.TouchEvent) => {
    setIsTouching(true);
    blockScroll();
    handleMovement(event);
  };

  const handleTouchMove = (event: React.TouchEvent) => {
    if (isTouching) {
      handleMovement(event);
    }
  };

  const handleTouchEnd = () => {
    setIsTouching(false);
    setMovementAngle(null);
    unblockScroll();
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

  const blockScroll = () => {
    document.body.style.overflow = 'hidden';
    document.addEventListener('touchmove', preventDefault, { passive: false });
  };

  const unblockScroll = () => {
    document.body.style.overflow = '';
    document.removeEventListener('touchmove', preventDefault);
  };

  const preventDefault = (e: TouchEvent) => {
    e.preventDefault();
  };

  return (
    <>
      {isMobile && (
        <div className="fixed bottom-10 w-full transform z-2 flex justify-around">
          {
            <div onTouchStart={handleShiftTouchStart} onTouchEnd={handleShiftTouchEnd}>
              <img src={MoveShiftImg} alt="" style={{ width: '100px', height: '100px' }} />
            </div>
          }
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

import { useAlert } from '@gear-js/react-hooks';
import { copyToClipboard } from '@ui/utils';
import clsx from 'clsx';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

import { Copy } from '@/assets/images';
import { Button, Modal } from '@/components';

import { useThrottle } from '../../hooks';
import { PlayersList } from '../players-list';

import styles from './start-game-modal.module.scss';

type Props = {
  totalPlayers: number;
  currentPlayers: number;
  buyIn: number;
  onClose: () => void;
  onStartGame: () => void;
};

const DRAG_THRESHOLD = 50;
const MAX_HEIGHT = 350;

const players = [
  {
    name: 'Player 1',
    avatar: 'https://avatar.iran.liara.run/public/27',
    isHost: true,
  },
  {
    name: 'Player 2',
    avatar: 'https://avatar.iran.liara.run/public/14',
  },
  {
    name: 'Player 3',
  },
  {
    name: 'Player 4',
  },
];

const StartGameModal = ({ totalPlayers, currentPlayers, buyIn, onClose, onStartGame }: Props) => {
  const alert = useAlert();

  const [isExpanded, setIsExpanded] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStartY, setDragStartY] = useState(0);
  const [dragOffset, setDragOffset] = useState(0);
  const playersRef = useRef<HTMLDivElement>(null);

  const heading = `Players ${currentPlayers}/${totalPlayers}`;
  const gameLink = 't.me/botname/bot?start=2765123';

  const seats = useMemo(() => Array.from({ length: totalPlayers }, (_, index) => index), [totalPlayers]);
  const isAdmin = true;
  const isSpectator = true;

  const handleDragStart = (e: React.MouseEvent | React.TouchEvent) => {
    setIsDragging(true);

    const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY;

    setDragStartY(clientY);
    setDragOffset(0);

    e.preventDefault();
  };

  const handleDrag = useCallback(
    (e: MouseEvent | TouchEvent) => {
      if (!isDragging) return;

      const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY;

      const offset = clientY - dragStartY;

      const maxOffset = isExpanded ? MAX_HEIGHT : 0;
      const minOffset = isExpanded ? 0 : -MAX_HEIGHT;
      const boundedOffset = Math.max(minOffset, Math.min(maxOffset, offset));
      setDragOffset(boundedOffset);

      if (playersRef.current) {
        const maxHeight = isExpanded ? MAX_HEIGHT : 0;
        playersRef.current.style.maxHeight = `${maxHeight - boundedOffset}px`;
      }
    },
    [isDragging, isExpanded, dragStartY],
  );

  const throttledHandleDrag = useThrottle(handleDrag, 50);

  const handleDragEnd = useCallback(() => {
    if (!isDragging) return;

    setIsDragging(false);
    if (isExpanded && dragOffset > DRAG_THRESHOLD) {
      setIsExpanded(false);
    } else if (!isExpanded && dragOffset < -DRAG_THRESHOLD) {
      setIsExpanded(true);
    }

    if (playersRef.current) {
      playersRef.current.style.maxHeight = '';
    }

    setDragOffset(0);
  }, [isDragging, isExpanded, dragOffset]);

  const handleGrabClick = () => {
    setIsExpanded(!isExpanded);
  };

  useEffect(() => {
    document.addEventListener('mousemove', throttledHandleDrag);
    document.addEventListener('touchmove', throttledHandleDrag);
    document.addEventListener('mouseup', handleDragEnd);
    document.addEventListener('touchend', handleDragEnd);

    return () => {
      document.removeEventListener('mousemove', throttledHandleDrag);
      document.removeEventListener('touchmove', throttledHandleDrag);
      document.removeEventListener('mouseup', handleDragEnd);
      document.removeEventListener('touchend', handleDragEnd);
    };
  }, [isDragging, dragStartY, isExpanded, throttledHandleDrag, handleDragEnd]);

  const onCopy = () => {
    void copyToClipboard({ value: gameLink, alert });
  };

  return (
    <Modal heading={heading} isDark showModalMode={false}>
      <div
        className={clsx(styles.grab)}
        role="button"
        tabIndex={0}
        onMouseDown={handleDragStart}
        onTouchStart={handleDragStart}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            handleGrabClick();
          }
        }}>
        <div className={styles.grabLine} />
      </div>

      <div className={clsx(isExpanded && styles.expanded)}>
        <div className={styles.gameId}>
          <span>{gameLink}</span>
          <Button color="plain" rounded size="small" className={styles.copyButton} onClick={onCopy}>
            <Copy />
          </Button>
        </div>

        <div className={clsx(styles.playersContainer, isExpanded && styles.visible)} ref={playersRef}>
          <PlayersList seats={seats} players={players} buyIn={buyIn} isAdmin={isAdmin} isSpectator={isSpectator} />
        </div>

        <div className={styles.buttons}>
          <Button color="danger" onClick={onClose}>
            Cancel game
          </Button>
          <Button color="primary" onClick={onStartGame}>
            Start game
          </Button>
        </div>
      </div>
    </Modal>
  );
};

export { StartGameModal };

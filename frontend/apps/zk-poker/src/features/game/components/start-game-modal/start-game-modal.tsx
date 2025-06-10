import { useAlert } from '@gear-js/react-hooks';
import { copyToClipboard } from '@ui/utils';
import clsx from 'clsx';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useParams } from 'react-router-dom';

import { Copy } from '@/assets/images';
import { Button, Modal } from '@/components';

import { useThrottle } from '../../hooks';
import { useKillMessage, useStartGameMessage } from '../../sails';
import { PlayersList } from '../players-list';

import styles from './start-game-modal.module.scss';

type Props = {
  participants: [`0x${string}`, Participant][];
  maxPlayers: number;
  isAdmin: boolean;
  isWaitingStart: boolean;
};

const DRAG_THRESHOLD = 30;
const MAX_HEIGHT = 350;

const StartGameModal = ({ participants, maxPlayers, isAdmin, isWaitingStart }: Props) => {
  const alert = useAlert();
  const { gameId } = useParams();
  const { startGameMessage, isPending: isStartGamePending } = useStartGameMessage();
  const { killMessage, isPending: isKillPending } = useKillMessage();

  const players = participants.map(([address, { name, balance }]) => ({
    address,
    name,
    avatar: undefined,
    balance: Number(balance),
  }));

  const [isExpanded, setIsExpanded] = useState(true);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStartY, setDragStartY] = useState(0);
  const [dragOffset, setDragOffset] = useState(0);
  const playersRef = useRef<HTMLDivElement>(null);

  const heading = `Players ${participants.length}/${maxPlayers}`;
  // ! TODO: move to env
  const tgBotName = 'zk-poker-bot';
  const gameLink = `t.me/${tgBotName}/bot?start=${gameId}`;

  const seats = useMemo(() => Array.from({ length: maxPlayers }, (_, index) => index), [maxPlayers]);

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
          <span className={styles.gameIdLink}>{gameLink}</span>
          <Button color="plain" rounded size="small" className={styles.copyButton} onClick={onCopy}>
            <Copy />
          </Button>
        </div>

        <div className={clsx(styles.playersContainer, isExpanded && styles.visible)} ref={playersRef}>
          <PlayersList seats={seats} players={players} isAdmin={isAdmin} />
        </div>

        {isAdmin && (
          <div className={styles.buttons}>
            <Button color="danger" onClick={() => killMessage()} disabled={isKillPending}>
              Cancel game
            </Button>
            <Button color="primary" onClick={() => startGameMessage()} disabled={isStartGamePending || !isWaitingStart}>
              Start game
            </Button>
          </div>
        )}
      </div>
    </Modal>
  );
};

export { StartGameModal };

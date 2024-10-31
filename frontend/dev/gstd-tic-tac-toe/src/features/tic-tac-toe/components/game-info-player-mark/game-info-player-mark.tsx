import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';
import styles from './game-info-player-mark.module.scss';
import { GameMark } from '../game-mark';
import type { Mark } from '../../types';
import { variantsPlayerMark } from '../../variants';
import { BaseComponentProps } from '@/app/types';

type GameSelectedFigureProps = BaseComponentProps & {
  mark: Mark;
  isNewGame: boolean;
};

export function GameInfoPlayerMark({ mark, className, isNewGame }: GameSelectedFigureProps) {
  const [isShown, setIsShown] = useState(false);

  useEffect(() => {
    if (isNewGame && isShown) setIsShown(false);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isNewGame]);

  return isNewGame && !isShown ? (
    <motion.div
      initial="enter"
      animate="center"
      variants={variantsPlayerMark}
      onAnimationComplete={() => {
        setIsShown(true);
      }}
      className={className}>
      <div className={styles.wrapper}>
        <div className={styles.box}>
          <GameMark mark={mark} />
        </div>
        <div className={styles.label}>
          <p>Your symbol</p>
        </div>
      </div>
    </motion.div>
  ) : null;
}

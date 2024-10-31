import { motion } from 'framer-motion';
import { Button } from '@/components/ui/button';
import styles from './game-cell.module.scss';
import { variantsGameMark } from '../../variants';
import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';

type GameFieldProps = BaseComponentProps & {
  disabled?: boolean;
  value: number;
  isLoading: boolean;
  onSelectCell: (value: number) => void;
};

export function GameCell({ children, className, disabled, value, isLoading, onSelectCell }: GameFieldProps) {
  const handleSelectCell = () => {
    onSelectCell(value);
  };

  return (
    <Button
      variant="text"
      className={clsx(styles.cell, className)}
      disabled={disabled || isLoading}
      onClick={handleSelectCell}>
      <motion.span
        initial="enter"
        animate="center"
        variants={variantsGameMark}
        custom={disabled ? 0 : 1}
        className={styles.mark}>
        {children}
      </motion.span>
    </Button>
  );
}

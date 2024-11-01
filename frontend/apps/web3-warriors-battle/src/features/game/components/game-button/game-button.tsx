import { VariantProps, cva } from 'class-variance-authority';
import clsx from 'clsx';
import { ButtonHTMLAttributes } from 'react';
import { GameButtonGlowSvg, LoaderIcon } from '../../assets/images';
import styles from './game-button.module.scss';

const variants = cva('', {
  variants: { color: { red: styles.red, green: styles.green, cyan: styles.cyan, black: styles.black } },
  defaultVariants: { color: 'red' },
});

type GameButtonProps = ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof variants> & {
    text: string;
    icon?: React.ReactNode;
    turnsBlocked?: number;
    pending?: boolean;
  };

export const GameButton = ({
  color,
  text,
  icon,
  className,
  turnsBlocked,
  disabled,
  pending,
  ...restProps
}: GameButtonProps) => {
  const isDisabled = Boolean(turnsBlocked) || disabled;
  const displayedText = turnsBlocked ? `Blocked for ${turnsBlocked} turns` : text;

  return (
    <button
      type="button"
      className={variants({
        color,
        className: clsx(styles.outer, className, turnsBlocked && styles.blocked, pending && styles.pending),
      })}
      disabled={isDisabled}
      {...restProps}>
      <div className={styles.inner}>
        {pending ? (
          <LoaderIcon />
        ) : (
          <>
            {!turnsBlocked && icon}
            <div>
              <span className={clsx(styles.text, styles.shadow)}>{displayedText}</span>
              <span className={clsx(styles.text, styles.stroke)}>{displayedText}</span>
              <span className={styles.text}>{displayedText}</span>
            </div>
          </>
        )}
        <GameButtonGlowSvg className={styles.glow} />
      </div>
      {color === 'black' && <div className={styles.blackAnimation}></div>}
    </button>
  );
};

import clsx from 'clsx';
import { HTMLAttributes } from 'react';

import styles from './segmented.module.scss';

type SegmentedOption = {
  label: React.ReactNode;
  value: string;
};

type SegmentedProps = HTMLAttributes<HTMLDivElement> & {
  options: SegmentedOption[];
  onChange: (value: string) => void;
  value: string;
};

const Segmented = ({ className, options, onChange, value }: SegmentedProps) => {
  return (
    <div className={clsx(styles.wrapper, className)}>
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          className={clsx(styles.option, value === option.value && styles.selected)}
          onClick={() => onChange(option.value)}>
          {option.label}
        </button>
      ))}
    </div>
  );
};

export { Segmented };

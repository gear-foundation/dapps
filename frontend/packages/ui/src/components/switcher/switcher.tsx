import { clsx } from 'clsx';
import styles from './switcher.module.css';

type Props = {
  disabled?: boolean;
  size?: 'small' | 'default' | 'large';
  defaultChecked?: boolean;
  checked?: boolean;
  onChange?: (isChecked: boolean) => void;
};

function Switcher({ checked, disabled, size = 'default', defaultChecked = false, onChange }: Props) {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onChange?.(e.target.checked);
  };

  return (
    <label className={clsx(styles.switch, styles[`switch-${size}`], disabled && styles[`switch-disabled`])}>
      <input
        type="checkbox"
        defaultChecked={defaultChecked}
        checked={checked}
        onChange={handleChange}
        disabled={disabled}
      />
      <span className={clsx(styles.slider, styles[`slider-${size}`], styles.round)}></span>
    </label>
  );
}

export { Switcher };

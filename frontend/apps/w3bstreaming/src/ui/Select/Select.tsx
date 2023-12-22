/* eslint-disable jsx-a11y/control-has-associated-label */
/* eslint-disable jsx-a11y/label-has-associated-control */
import styles from './Select.module.scss';
import { SelectProps } from './Select.interfaces';

function Select({ label, size, placeholder, options, ...props }: SelectProps) {
  return (
    <div className={styles.select}>
      <select className={styles['select-text']} {...props}>
        <option value="" disabled selected />
        {options.map((option) => (
          <option value={option.value}>{option.label}</option>
        ))}
      </select>
      <span className={styles['select-highlight']} />
      <span className={styles['select-bar']} />
      <label className={styles['select-label']}>{label}</label>
    </div>
  );
}

export { Select };

/* eslint-disable jsx-a11y/control-has-associated-label */
/* eslint-disable jsx-a11y/label-has-associated-control */
import styles from './Select.module.scss';
import { SelectProps } from './Select.interfaces';

function Select({ label, size, placeholder, ...props }: SelectProps) {
  return (
    <div className={styles.select}>
      <select className={styles['select-text']} required {...props}>
        <option value="" disabled selected />
        <option value="1">Option 1</option>
        <option value="2">Option 2</option>
        <option value="3">Option 3</option>
      </select>
      <span className={styles['select-highlight']} />
      <span className={styles['select-bar']} />
      <label className={styles['select-label']}>{label}</label>
    </div>
  );
}

export { Select };

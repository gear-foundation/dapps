import { SelectProps } from './Select.interfaces';
import styles from './Select.module.scss';

function Select({ label, options, ...props }: SelectProps) {
  return (
    <div className={styles.select}>
      <select className={styles['select-text']} required {...props}>
        <option value="" disabled />
        {options.map((option) => (
          <option value={option.value} key={`${option.label}-${option.value}`}>
            {option.label}
          </option>
        ))}
      </select>
      <span className={styles['select-highlight']} />
      <span className={styles['select-bar']} />
      <label className={styles['select-label']}>{label}</label>
    </div>
  );
}

export { Select };

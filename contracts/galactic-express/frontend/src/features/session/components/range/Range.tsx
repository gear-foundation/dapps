import { Input } from '@gear-js/ui';
import { InputHTMLAttributes } from 'react';
import styles from './Range.module.scss';

type Props = InputHTMLAttributes<HTMLInputElement> & {
  label: string;
};

function Range({ label, value, onChange }: Props) {
  const style = { backgroundImage: `linear-gradient(to right, #2bd071 ${value}%, #2c2b30 ${value}%)` };

  return (
    <div>
      <Input type="number" label={label} className={styles.input} value={value} onChange={onChange} />

      <input type="range" className={styles.range} value={value} onChange={onChange} style={style} />

      <p className={styles.labels}>
        <span>0</span>
        <span>100</span>
      </p>
    </div>
  );
}

export { Range };

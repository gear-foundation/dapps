import { Input } from '@gear-js/ui';
import { InputHTMLAttributes } from 'react';
import styles from './Range.module.scss';

type Props = Omit<InputHTMLAttributes<HTMLInputElement>, 'size' | 'color'> & {
  label: string;
};

function Range(props: Props) {
  const { value } = props;

  const style = { backgroundImage: `linear-gradient(to right, #2bd071 ${value}%, #2c2b30 ${value}%)` };

  return (
    <div>
      <Input type="number" min={0} max={100} className={styles.input} {...props} />
      <input type="range" min={0} max={100} className={styles.range} style={style} {...props} />

      <p className={styles.labels}>
        <span>0</span>
        <span>100</span>
      </p>
    </div>
  );
}

export { Range };

import { useState } from 'react';
import { MultiSwithProps, Option } from './MultiSwitch.interfaces';
import styles from './MultiSwitch.module.scss';
import { cx } from '@/utils';

function MultiSwitch({ options, defaultSelected, onSelectOption }: MultiSwithProps) {
  const [selected, setSelected] = useState<string>(defaultSelected || options[0].name);

  const handleChange = (option: Option) => {
    if (selected !== option.name) {
      setSelected(option.name);
      onSelectOption?.(option);
    }
  };

  return (
    <div className={cx(styles.multiswitch)}>
      {options.map((option) => (
        <button
          key={option.name}
          className={cx(styles.btn, selected === option.name ? styles.selected : '')}
          onClick={() => handleChange(option)}>
          {option.name}
        </button>
      ))}
    </div>
  );
}

export { MultiSwitch };

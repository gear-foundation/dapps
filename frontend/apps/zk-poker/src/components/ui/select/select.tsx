import clsx from 'clsx';
import { useRef, useState, useEffect } from 'react';

import { ChevronDown } from '@/assets/images';

import { inputStyles } from '../input';

import styles from './select.module.scss';

type Option = {
  value: string | number;
  label: string;
};

type SelectProps = {
  id?: string;
  name?: string;
  value: string | number;
  options: Option[];
  onChange: (value: string | number) => void;
  className?: string;
};

const Select = ({ name, value, options, onChange, className, id }: SelectProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const selectRef = useRef<HTMLDivElement>(null);

  const selectedOption = options.find((option) => option.value === value) || options[0];

  const handleOptionClick = (optionValue: string | number) => {
    onChange(optionValue);
    setIsOpen(false);
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (selectRef.current && !selectRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  return (
    <div className={clsx(styles.selectContainer, className)} ref={selectRef}>
      <button
        type="button"
        className={clsx(inputStyles.input, styles.selectTrigger, isOpen && styles.open)}
        onClick={() => setIsOpen(!isOpen)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            setIsOpen(!isOpen);
          }
        }}
        aria-expanded={isOpen}
        aria-haspopup="listbox"
        id={id}>
        <span>{selectedOption.label}</span>
        <ChevronDown className={styles.caret} />
      </button>

      {isOpen && (
        <div className={styles.options} role="listbox" aria-label="Select options">
          {options.map((option) => (
            <button
              type="button"
              key={option.value}
              className={clsx(styles.option, option.value === value && styles.selected)}
              onClick={() => handleOptionClick(option.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  handleOptionClick(option.value);
                }
              }}
              role="option"
              aria-selected={option.value === value}>
              {option.label}
            </button>
          ))}
        </div>
      )}
      <input type="hidden" name={name} value={value} />
    </div>
  );
};

export { Select };

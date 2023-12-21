import { SelectHTMLAttributes } from 'react';

export interface SelectProps extends Omit<SelectHTMLAttributes<HTMLSelectElement>, 'size'> {
  size?: 'large' | 'medium' | 'small';
  label?: string;
  options: {
    label: string;
    value: string;
  }[];
}

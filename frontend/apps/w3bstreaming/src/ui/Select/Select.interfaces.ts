import { SelectHTMLAttributes } from 'react';

export interface SelectProps extends Omit<SelectHTMLAttributes<HTMLSelectElement>, 'size'> {
  label?: string;
  options: {
    label: string;
    value: string;
  }[];
}

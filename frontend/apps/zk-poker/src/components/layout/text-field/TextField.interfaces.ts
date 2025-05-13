import { InputHTMLAttributes } from 'react';

export interface TextFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'size'> {
  label?: string;
  variant?: 'default' | 'active';
  icon?: React.ReactNode;
  theme?: 'light' | 'dark';
}

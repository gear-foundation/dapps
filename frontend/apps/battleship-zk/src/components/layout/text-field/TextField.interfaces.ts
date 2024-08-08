import { InputHTMLAttributes } from 'react';

export interface TextFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'size'> {
  size?: 'large' | 'medium' | 'small';
  label?: string;
  variant?: 'default' | 'active';
  icon?: React.ReactNode;
  theme?: 'light' | 'dark';
}

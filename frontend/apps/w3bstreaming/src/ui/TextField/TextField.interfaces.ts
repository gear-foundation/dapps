import { InputHTMLAttributes } from 'react';

export interface TextFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'size'> {
  size?: 'large' | 'medium' | 'small';
  label?: string;
}

import { ButtonHTMLAttributes } from 'react';

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  label?: string;
  variant: 'primary' | 'outline' | 'icon' | 'text';
  size?: 'large' | 'medium' | 'small';
  icon?: string;
}

import { TextareaHTMLAttributes } from 'react';

export interface InputProps extends Omit<TextareaHTMLAttributes<HTMLTextAreaElement>, 'size'> {
  size?: 'large' | 'medium' | 'small';
}

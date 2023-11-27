import { ChangeEvent } from 'react';

export interface SearchProps {
  placeholder?: string;
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

import { ReactNode } from 'react';

declare global {
  type BaseComponentProps = {
    children?: ReactNode;
    className?: string;
  };
}

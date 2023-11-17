import { ReactNode } from 'react';
import { LinkProps as RouterLinkProps } from 'react-router-dom';

export interface LinkProps extends RouterLinkProps {
  to: string;
  children: ReactNode;
}

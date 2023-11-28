import { PropsWithChildren } from 'react';

export type BaseComponentProps = PropsWithChildren & {
  className?: string;
};

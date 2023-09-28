import { ReactNode } from 'react';

export type ModalProps = {
  heading: string;
  children: ReactNode;
  onClose: () => void;
};

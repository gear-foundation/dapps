import { ModalProps } from '@/components/Modal/Modal.interface';

export interface TransactionModalProps extends ModalProps {
  fee: string;
  name: string;
  addressTo?: string;
  addressFrom: string;
  onAbort?: () => void;
  onConfirm: () => void;
}

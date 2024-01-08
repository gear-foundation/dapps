export interface WalletModalProps {
  open: boolean;
  setOpen(value: boolean): void;
  onClose: () => void;
}

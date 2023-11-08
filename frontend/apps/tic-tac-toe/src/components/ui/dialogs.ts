import { WalletModalProps } from '@/features/wallet/components/wallet-modal';
import { MobileMenuDialogProps } from '@/components/layout/header/mobile-menu';
import { ComponentType, lazy } from 'react';

export interface IDialogsLibrary {
  WalletModal: WalletModalProps;
  MobileMenuDialog: MobileMenuDialogProps;
}

export const DialogsLibrary: Record<keyof IDialogsLibrary, any> = {
  WalletModal: lazy<ComponentType<WalletModalProps>>(() =>
    import('@/features/wallet/components/wallet-modal').then(({ WalletModal }) => ({ default: WalletModal })),
  ),
  MobileMenuDialog: lazy<ComponentType<MobileMenuDialogProps>>(() =>
    import('@/components/layout/header/mobile-menu').then(({ MobileMenuDialog }) => ({
      default: MobileMenuDialog,
    })),
  ),
};

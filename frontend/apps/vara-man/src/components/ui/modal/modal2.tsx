import { cn } from '@/app/utils';
import * as Dialog from '@radix-ui/react-dialog';
import { ReactNode } from 'react';

export function Modal({
  open,
  onOpenChange,
  children,
}: {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  children: ReactNode;
}) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      {children}
    </Dialog.Root>
  );
}

function ModalContent({ children, classNameContent }: { children: ReactNode; classNameContent?: string }) {
  return (
    <Dialog.Portal>
      <Dialog.Overlay className="fixed inset-0 z-2 bg-black/50 data-[state=closed]:animate-[dialog-overlay-hide_200ms] data-[state=open]:animate-[dialog-overlay-show_200ms] backdrop-blur-lg" />
      <Dialog.Content
        className={cn(
          'fixed left-1/2 md:top-1/2 bottom-0 rounded-t-lg z-2 w-full max-w-md -translate-x-1/2 md:-translate-y-1/2 rounded-md bg-white p-8 text-gray-900 shadow data-[state=closed]:animate-[dialog-content-hide_200ms] data-[state=open]:animate-[dialog-content-show_200ms]',
          classNameContent,
        )}>
        {children}
      </Dialog.Content>
    </Dialog.Portal>
  );
}

Modal.Button = Dialog.Trigger;
Modal.Close = Dialog.Close;
Modal.Content = ModalContent;

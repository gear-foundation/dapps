'use client';

import * as DialogPrimitive from '@radix-ui/react-dialog';
import { m } from 'framer-motion';
import * as React from 'react';
import clsx from 'clsx';
import styles from './dialog.module.scss';
import { CrossIcon } from '@/assets/images';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/modal.variants';

const Dialog = DialogPrimitive.Root;

const DialogTrigger = DialogPrimitive.Trigger;

const DialogPortal = ({ ...props }: DialogPrimitive.DialogPortalProps) => <DialogPrimitive.Portal {...props} />;
DialogPortal.displayName = DialogPrimitive.Portal.displayName;

const DialogOverlay = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Overlay>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>
>(({ ...props }, ref) => <DialogPrimitive.Overlay {...props} ref={ref} />);
DialogOverlay.displayName = DialogPrimitive.Overlay.displayName;

const DialogContent = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>
>(({ ...props }, ref) => <DialogPrimitive.Content ref={ref} {...props} />);
DialogContent.displayName = DialogPrimitive.Content.displayName;

const DialogHeader = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <div className={clsx('flex flex-col space-y-2 rounded-lg text-center sm:text-left', className)} {...props} />
);
DialogHeader.displayName = 'DialogHeader';

const DialogFooter = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <div className={clsx('flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2', className)} {...props} />
);
DialogFooter.displayName = 'DialogFooter';

const DialogTitle = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Title>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Title>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Title ref={ref} className={clsx('text-2xl font-bold leading-8', className)} {...props} />
));
DialogTitle.displayName = DialogPrimitive.Title.displayName;

const DialogDescription = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Description>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Description>
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Description ref={ref} className={clsx('text-sm text-neutral-400', className)} {...props} />
));
DialogDescription.displayName = DialogPrimitive.Description.displayName;

const DialogOverlayFramer = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Overlay>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>
>(({ className, children, ...props }, ref) => (
  <DialogPrimitive.Overlay {...props} ref={ref} asChild>
    <m.div
      className={clsx(
        'fixed inset-0 z-50 bg-black/20 backdrop-blur transition-all duration-100 data-[state=closed]:animate-out data-[state=open]:fade-in data-[state=closed]:fade-out',
        className,
      )}
      variants={variantsOverlay}
      initial="enter"
      animate="center"
      exit="exit"
    />
  </DialogPrimitive.Overlay>
));
DialogOverlayFramer.displayName = DialogPrimitive.Overlay.displayName;

const DialogContentFramer = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>
>(({ className, children, ...props }, ref) => (
  <DialogPrimitive.Content ref={ref} {...props} asChild>
    <m.div
      variants={variantsPanel}
      initial="enter"
      animate="center"
      exit="exit"
      className={clsx('fixed z-50 py-7.5 px-8 w-full bg-[#F6F8F8] shadow-xl rounded', className ?? 'max-w-md')}>
      {children}

      <DialogPrimitive.Close className="absolute top-7.5 right-6.5 rounded-sm transition-opacity hover:opacity-60">
        <CrossIcon className="h-7.5 w-7.5" />
        <span className="sr-only">Close</span>
      </DialogPrimitive.Close>
    </m.div>
  </DialogPrimitive.Content>
));
DialogContentFramer.displayName = DialogPrimitive.Content.displayName;

const DialogClose = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Close>,
  React.ComponentPropsWithoutRef<typeof DialogPrimitive.Close>
>(({ className, children, ...props }, ref) => (
  <DialogPrimitive.Close ref={ref} className={className} {...props}>
    {children}
  </DialogPrimitive.Close>
));
DialogClose.displayName = DialogPrimitive.Content.displayName;

export {
  Dialog,
  DialogTrigger,
  DialogPortal,
  DialogOverlay,
  DialogOverlayFramer,
  DialogContent,
  DialogContentFramer,
  DialogHeader,
  DialogFooter,
  DialogTitle,
  DialogDescription,
  DialogClose,
};

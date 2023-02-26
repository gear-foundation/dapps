import * as ScrollArea from '@radix-ui/react-scroll-area';
import { ReactNode } from 'react';
import clsx from 'clsx';

type Props = {
  children: ReactNode;
  className?: string;
};
export const ScrollWrapper = ({ children, className }: Props) => {
  return (
    <ScrollArea.Root className={clsx('flex flex-col overflow-hidden', className ?? 'max-h-80 pr-5 -mr-5')} type="auto">
      <ScrollArea.Viewport className="grow">{children}</ScrollArea.Viewport>
      <ScrollArea.Scrollbar
        className="mr-px flex select-none touch-none bg-white/10 transition-colors duration-[160ms] ease-out hover:bg-white/20 data-[orientation=vertical]:w-px data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-px"
        orientation="vertical">
        <ScrollArea.Thumb className="flex-1 bg-white rounded-[10px] relative -mx-px" />
      </ScrollArea.Scrollbar>
      <ScrollArea.Scrollbar
        className="flex select-none touch-none p-0.5 bg-white/10 transition-colors duration-[160ms] ease-out hover:bg-white/20 data-[orientation=vertical]:w-2.5 data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-2.5"
        orientation="horizontal">
        <ScrollArea.Thumb className="flex-1 bg-mauve10 rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]" />
      </ScrollArea.Scrollbar>
      <ScrollArea.Corner className="bg-white/20" />
    </ScrollArea.Root>
  );
};

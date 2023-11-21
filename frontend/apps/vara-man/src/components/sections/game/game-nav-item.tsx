import { cn } from '@/app/utils';
import { ReactNode } from 'react';

type GameNavItemProps = BaseComponentProps & {
  icon: ReactNode;
};
export const GameNavItem = ({ children, className, icon }: GameNavItemProps) => (
  <div
    className={cn(
      'flex items-center text-[var(--stats-theme)] font-semibold tracking-wider text-base leading-none',
      className,
    )}>
    <div className="relative z-1">{icon}</div>
    <div className="relative flex items-center justify-center -ml-4 w-17.5 bg-white/[1%] shadow-[var(--stats-theme)] shadow-[inset_0_0_4px] h-6 pr-3 pl-6 rounded-r-full overflow-hidden">
      {children}
      <span className="absolute top-full -mt-0.5 w-2.5 h-2.5 rounded-full bg-[var(--stats-theme)] shadow-[0_0_10px_1px_var(--stats-theme)]" />
    </div>
  </div>
);

import { WarningIcon } from '@/assets/images'
import clsx from 'clsx'

type InfoCardProps = BaseComponentProps & {}

export function InfoCard({ children, className }: InfoCardProps) {
  return (
    <div
      className={clsx(
        'flex items-center py-4 rounded-xl bg-gradient-to-r from-primary-600/[.17] to-transparent',
        className
      )}
    >
      <div className="p-5.5">
        <WarningIcon />
      </div>
      <div className="font-medium text-base leading-[22px] font-kanit tracking-[0.64px]">
        {children}
      </div>
    </div>
  )
}

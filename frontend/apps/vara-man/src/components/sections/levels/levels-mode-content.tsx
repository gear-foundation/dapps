import { cn } from '@/app/utils';

type LevelsModeContentProps = BaseComponentProps & {
  title: string;
  colorText: string;
};

export function LevelsModeContent({ children, colorText, title, className }: LevelsModeContentProps) {
  return (
    <div className="xl:mt-5 xxl:mt-15 xl:-mr-15 xl:ml-15">
      <div className={cn('typo-h2', colorText)}>
        <i className="font-extralight">{title}</i> <b className="text-white">level</b>
      </div>
      {children}
    </div>
  );
}

import styles from './typography.module.scss';
import clsx from 'clsx';
import { textVariants } from '@/components/ui/text/text';

type HelpDescriptionProps = React.PropsWithChildren & { className?: string };

export function HelpDescription({ children, className }: HelpDescriptionProps) {
  return <div className={clsx(styles.description, textVariants({ size: 'lg', className }))}>{children}</div>;
}

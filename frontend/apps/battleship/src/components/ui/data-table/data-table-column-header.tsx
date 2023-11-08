import { HeaderContext } from '@tanstack/react-table';
import clsx from 'clsx';
import styles from '@/components/ui/data-table/table.module.scss';
import { DataTableHeaderActions } from '@/components/ui/data-table/data-table-header-actions';

interface DataTableColumnHeaderProps<TData, TValue> extends React.HTMLAttributes<HTMLDivElement> {
  data: HeaderContext<TData, TValue>;
  title: string;
}

export function DataTableColumnHeader<TData, TValue>({
  data: { column, header },
  title,
  className,
}: DataTableColumnHeaderProps<TData, TValue>) {
  if (!column.getCanSort()) {
    return <div className={clsx(className)}>{title}</div>;
  }

  return (
    <div className={clsx(styles['with-sorting'], 'group', className)}>
      <div>{title}</div>
      <DataTableHeaderActions header={header} title={title} />
    </div>
  );
}

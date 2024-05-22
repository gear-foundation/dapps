import { Table } from '@tanstack/react-table';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import styles from './table.module.scss';
import clsx from 'clsx';

type TablePageSizingProps<T> = BaseComponentProps & {
  table: Table<T>;
};

export function DataTablePageSizing<TData>({ table }: TablePageSizingProps<TData>) {
  return (
    <div className={styles['table-page-sizing']}>
      <Select
        value={`${table.getState().pagination.pageSize}`}
        onValueChange={(value) => {
          table.setPageSize(Number(value));
        }}>
        <SelectTrigger className={styles.value}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {[10, 20, 30, 40, 50].map((option, i) => (
            <SelectItem value={String(option)} key={option}>
              1–{option}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      <div className={clsx(styles.count, styles.muted)}>per page</div>
    </div>
  );
}

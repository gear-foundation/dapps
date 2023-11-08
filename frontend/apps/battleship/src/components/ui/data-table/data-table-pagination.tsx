import { Table } from '@tanstack/react-table';
import { ChevronsRight, ChevronRight } from '@/assets/images';
import styles from './table.module.scss';
import { Button } from '../button';
import clsx from 'clsx';
import { Text } from '@/components/ui/text';

type TablePaginationProps<T> = {
  table: Table<T>;
};

export function DataTablePagination<TData>({ table }: TablePaginationProps<TData>) {
  return (
    <div className={styles['table-pagination']}>
      <div className={styles['pagination-nav']}>
        <Button
          variant="outline"
          className={clsx(styles.action, styles.left)}
          onClick={() => table.setPageIndex(0)}
          disabled={!table.getCanPreviousPage()}>
          <ChevronsRight width={20} height={20} />
        </Button>
        <Button
          variant="outline"
          className={clsx(styles.action, styles.left)}
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}>
          <ChevronRight width={20} height={20} />
        </Button>
      </div>

      <Text className={styles['pagination-pages']}>
        <span className={styles.pageNumber}>{table.getState().pagination.pageIndex + 1}</span>
        <span className={styles.pagesCount}>out of {table.getPageCount()}</span>
      </Text>

      <div className={styles['pagination-nav']}>
        <Button
          variant="outline"
          className={styles.action}
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}>
          <ChevronRight width={20} height={20} />
        </Button>
        <Button
          variant="outline"
          className={styles.action}
          onClick={() => table.setPageIndex(table.getPageCount() - 1)}
          disabled={!table.getCanNextPage()}>
          <ChevronsRight width={20} height={20} />
        </Button>
      </div>
    </div>
  );
}

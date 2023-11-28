import { Table } from '@tanstack/react-table'
import {
  ChevronsLeft,
  ChevronLeft,
  ChevronsRight,
  ChevronRight,
} from '@/assets/images'
import styles from './table.module.scss'

type TablePaginationProps<T> = {
  table: Table<T>
}

export function DataTablePagination<TData>({
  table,
}: TablePaginationProps<TData>) {
  return (
    <div className={styles['table-pagination']}>
      <div className={styles.actions}>
        <button
          className={styles.action}
          onClick={() => table.setPageIndex(0)}
          disabled={!table.getCanPreviousPage()}
        >
          <ChevronsLeft width={20} height={20} />
        </button>
        <button
          className={styles.action}
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          <ChevronLeft width={20} height={20} />
        </button>
      </div>

      <div className={styles['page-info']}>
        <b>{table.getState().pagination.pageIndex + 1}</b>
        <span className={styles.muted}>out of {table.getPageCount()}</span>
      </div>

      <div className={styles.actions}>
        <button
          className={styles.action}
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          <ChevronRight width={20} height={20} />
        </button>
        <button
          className={styles.action}
          onClick={() => table.setPageIndex(table.getPageCount() - 1)}
          disabled={!table.getCanNextPage()}
        >
          <ChevronsRight width={20} height={20} />
        </button>
      </div>
    </div>
  )
}

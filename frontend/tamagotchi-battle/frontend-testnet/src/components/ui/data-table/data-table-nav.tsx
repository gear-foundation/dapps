import { DataTablePagination } from '@/components/ui/data-table/data-table-pagination'
import type { Table as TableType } from '@tanstack/table-core/build/lib/types'
import { DataTablePageSizing } from './data-table-page-sizing'
import styles from './table.module.scss'

type DataTableNavProps<T> = {
  table: TableType<T>
}

export function DataTableNav<T>({ table }: DataTableNavProps<T>) {
  return (
    <div className={styles['table-navigation']}>
      <DataTablePageSizing table={table} />
      <DataTablePagination table={table} />
    </div>
  )
}

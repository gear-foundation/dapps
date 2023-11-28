import { flexRender, Table as TableType } from '@tanstack/react-table'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

import styles from './table.module.scss'

type LkContentTableLayoutProps<T> = BaseComponentProps & {
  table: TableType<T>
  isLoading?: boolean
  className?: string
}

export function DataTable<TData>({
  table,
  isLoading,
  className,
}: LkContentTableLayoutProps<TData>) {
  return (
    <Table className={className}>
      <TableHeader>
        {table.getHeaderGroups().map((headerGroup) => (
          <TableRow key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <TableHead
                key={header.id}
                colSpan={header.colSpan}
                scope="col"
                style={{
                  width:
                    header.getSize() !== 150 ? header.getSize() : undefined,
                }}
              >
                {header.isPlaceholder
                  ? null
                  : flexRender(
                      header.column.columnDef.header,
                      header.getContext()
                    )}
              </TableHead>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {isLoading ? (
          Array.from({ length: table.getState().pagination.pageSize }).map(
            (row, i) => (
              <TableRow key={i}>
                {Array.from({ length: table.getAllColumns().length }).map(
                  (row, i) => (
                    <TableCell key={i}>
                      <div />
                    </TableCell>
                  )
                )}
              </TableRow>
            )
          )
        ) : table.getRowModel().rows?.length > 0 ? (
          table.getRowModel().rows.map((row) => (
            <TableRow key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <TableCell key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </TableCell>
              ))}
            </TableRow>
          ))
        ) : (
          <TableRow>
            <TableCell
              colSpan={table.getAllColumns().length}
              className={styles.notFound}
            >
              No results.
            </TableCell>
          </TableRow>
        )}
      </TableBody>
    </Table>
  )
}

// {/*<pre>*/}
// {/*  <code>{JSON.stringify(table.getState(), null, 2)}</code>*/}
// {/*</pre>*/}

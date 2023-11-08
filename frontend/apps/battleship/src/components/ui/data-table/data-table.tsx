import { flexRender, Table as TableType } from '@tanstack/react-table';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';

import styles from './table.module.scss';

type LkContentTableLayoutProps<T> = BaseComponentProps & {
  table: TableType<T>;
  isLoading?: boolean;
  className?: string;
};

export function DataTable<TData>({ table, isLoading, className, children }: LkContentTableLayoutProps<TData>) {
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
                  width: header.getSize() !== 150 ? header.getSize() : undefined,
                  minWidth: header.getContext().column.columnDef.minSize,
                }}>
                {header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext())}
              </TableHead>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {isLoading ? (
          <TableLoadingState table={table} />
        ) : table.getRowModel().rows?.length > 0 ? (
          children ? (
            children
          ) : (
            table.getRowModel().rows.map((row) => (
              <TableRow key={row.id} className={'bg-red-300'}>
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>{flexRender(cell.column.columnDef.cell, cell.getContext())}</TableCell>
                ))}
              </TableRow>
            ))
          )
        ) : (
          <TableNoResults table={table} />
        )}
      </TableBody>
    </Table>
  );
}

// {/*<pre>*/}
// {/*  <code>{JSON.stringify(table.getState(), null, 2)}</code>*/}
// {/*</pre>*/}

function TableNoResults<TData>({ table }: { table: TableType<TData> }) {
  return (
    <TableRow>
      <TableCell
        colSpan={table.getAllColumns().length}
        rowSpan={table.getState().pagination.pageSize}
        className={styles.notFound}>
        No results.
      </TableCell>
    </TableRow>
  );
}

function TableLoadingState<TData>({ table }: { table: TableType<TData> }) {
  return (
    <>
      {Array.from({ length: table.getState().pagination.pageSize }).map((row, i) => (
        <TableRow key={i}>
          {Array.from({ length: table.getAllColumns().length }).map((row, i) => (
            <TableCell key={i}>
              <div />
            </TableCell>
          ))}
        </TableRow>
      ))}
    </>
  );
}

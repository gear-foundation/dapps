import styles from './leaderboard.module.scss'
import {
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  PaginationState,
  SortingState,
  useReactTable,
} from '@tanstack/react-table'
import { useGame } from '@/features/tic-tac-toe/hooks'
import { useState } from 'react'
import { DataTable } from '@/components/ui/data-table'
import { DataTableNav } from '@/components/ui/data-table/data-table-nav'
import {
  leaderboardColumns,
  searchByNameOrAddressFilter,
} from './leaderboard.data'
import { LeaderboardSearchField } from './leaderboard-search-field'

export function Leaderboard() {
  const { leaderboard } = useGame()
  const [sorting, setSorting] = useState<SortingState>([])
  const [globalFilter, setGlobalFilter] = useState<string>('')
  const [pagination, setPagination] = useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10,
  })

  const table = useReactTable({
    data: leaderboard,
    columns: leaderboardColumns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    state: {
      pagination,
      sorting,
      globalFilter,
    },
    filterFns: {
      searchByNameOrAddress: searchByNameOrAddressFilter,
    },
    globalFilterFn: searchByNameOrAddressFilter,
    pageCount: Math.ceil(leaderboard.length / pagination.pageSize) || 0,
    onSortingChange: setSorting,
    onPaginationChange: setPagination,
    onGlobalFilterChange: setGlobalFilter,
  })

  const limit = table.getState().pagination.pageSize
  const pageMax =
    !!limit && !!leaderboard.length ? Math.ceil(leaderboard.length / limit) : 0

  return (
    <div className={styles.content}>
      <div className={styles.header}>
        <h2 className={styles.title}>
          Total participants: {leaderboard.length}
        </h2>
        <div className={styles.search}>
          <LeaderboardSearchField
            onSearch={(value) => setGlobalFilter(String(value))}
            placeholder="Search"
            disabled={!leaderboard.length}
          />
        </div>
      </div>

      <DataTable table={table} isLoading={false} className={styles.table} />
      {pageMax > 1 && <DataTableNav table={table} />}
    </div>
  )
}

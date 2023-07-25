import { ColumnDef, FilterFn } from '@tanstack/react-table'
import { ILeaderboardPlayer } from '@/features/tic-tac-toe/types'
import clsx from 'clsx'
import styles from './leaderboard.module.scss'
import { prettyAddress } from '@/app/utils'
import { DataTableColumnHeader } from '@/components/ui/data-table/data-table-column-header'
import { PointsBalance } from '@/components/ui/balance'
import { calculateWinRate } from '@/features/tic-tac-toe/utils'

export const leaderboardColumns: ColumnDef<ILeaderboardPlayer>[] = [
  {
    accessorFn: (row) => row.position,
    id: 'position',
    cell: (info) => (
      <div
        className={clsx(
          styles.place,
          info.getValue() === 1 && styles.first,
          info.getValue() === 2 && styles.second,
          info.getValue() === 3 && styles.third
        )}
      >
        <>{info.getValue()}</>
      </div>
    ),
    header: '#',
    enableSorting: false,
    enableGlobalFilter: false,
  },
  {
    accessorFn: (row) => row.name,
    id: 'name',
    cell: (info) => (
      <div className={styles.name}>
        <strong>
          <>{info.getValue()}</>
        </strong>{' '}
        (
        <span className={styles.address}>
          {prettyAddress(info.row.original.address)}
        </span>
        )
      </div>
    ),
    header: (ctx) => <DataTableColumnHeader data={ctx} title="Name" />,
  },
  {
    accessorFn: (row) => row.totalWins,
    id: 'totalWins',
    cell: (info) => (
      <span className="">
        <>{info.getValue()}</>
      </span>
    ),
    header: (ctx) => <DataTableColumnHeader data={ctx} title="Victories" />,
    enableGlobalFilter: false,
    sortingFn: 'alphanumeric',
  },
  {
    accessorFn: (row) => row.totalWins,
    id: 'winrate',
    cell: ({ row }) => (
      <div className={styles.winrate}>
        {calculateWinRate(row.original.totalWins, row.original.totalGames)}%
      </div>
    ),
    header: (ctx) => <DataTableColumnHeader data={ctx} title="Success Rate" />,
    enableGlobalFilter: false,
    sortingFn: 'alphanumeric',
  },
  {
    accessorFn: (row) => row.points,
    id: 'points',
    cell: (info) => (
      <div className={styles['points-container']}>
        <PointsBalance
          value={info.row.original.points}
          className={styles.points}
        />
      </div>
    ),
    header: () => <div className={styles['header-points']}>Total points</div>,
    enableSorting: false,
    enableGlobalFilter: false,
    sortingFn: 'alphanumeric',
  },
]

export const searchByNameOrAddressFilter: FilterFn<ILeaderboardPlayer> = (
  rows,
  columnIds,
  filterValue
) => {
  // return the filtered rows
  const { address, name } = rows.original
  return (
    address.toLowerCase().includes(filterValue.toLowerCase()) ||
    name.toLowerCase().includes(filterValue.toLowerCase())
  )
}

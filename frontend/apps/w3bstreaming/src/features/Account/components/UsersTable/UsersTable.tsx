import { useState } from 'react';
import styles from './UsersTable.module.scss';
import { EmptyTableContentProps, UsersTableProps } from './UsersTable.interfaces';
import { cx } from '@/utils';
import { Table } from '@/ui';
import img from '@/assets/icons/no-avatar-user-img.png';
import { CellValue, TableRow } from '@/ui/Table/Table.interfaces';
import { SubscribeModal } from '../SubscribeModal';

function EmptyTableContent({ name }: EmptyTableContentProps) {
  return (
    <td>
      <h3 className={cx(styles['empty-user-table-title'])}>No {name}</h3>
      <span className={cx(styles['empty-user-table-caption'])}>You don&apos;t have {name} yet ...</span>
    </td>
  );
}

function UsersTable({ data, columns, searchParams, sortedColumns, name }: UsersTableProps) {
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  const [idToUnsubscribe, setIdToUnsubscribe] = useState<string | null>(null);

  const handleUnsibscribe = (id: string | number) => {
    setIdToUnsubscribe(() => id as string);
    setIsModalOpen(() => true);
  };

  const handleCloseModal = () => {
    setIdToUnsubscribe(() => null);
    setIsModalOpen(() => false);
  };

  const cell = (columnName: string | number, value: CellValue, row: TableRow) => {
    if (columnName === 'Action') {
      return (
        <button className={cx(styles['unsubscribe-cell'])} onClick={() => handleUnsibscribe(row.id)}>
          Unsubscribe
        </button>
      );
    }

    if (columnName === 'Streamer' || columnName === 'User') {
      return (
        <div className={cx(styles['streamer-cell'])}>
          <img src={(row.img as string) || img} alt="img" className={cx(styles['user-image'])} />
          <span className={cx(styles['streamer-cell-name'])}>{value}</span>
        </div>
      );
    }

    return value;
  };

  return (
    <div className={cx(styles.table)}>
      <Table
        rows={data}
        pagination={{ rowsPerPage: 10 }}
        columns={columns}
        renderCell={cell}
        className={{
          headerCell: cx(styles['header-cell']),
          cell: cx(styles.cell),
        }}
        searchParams={{ ...searchParams, placeholder: 'Search transactions' }}
        sortedColumns={sortedColumns}
        renderEmpty={<EmptyTableContent name={name || ''} />}
      />
      {isModalOpen && <SubscribeModal type="unsubscribe" speakerId={idToUnsubscribe} onClose={handleCloseModal} />}
    </div>
  );
}

export { UsersTable };

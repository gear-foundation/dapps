import { useState } from 'react';
import styles from './UsersTable.module.scss';
import { UsersTableProps } from './UsersTable.interfaces';
import { cx } from '@/utils';
import { Table } from '@/ui';
import img from '@/assets/icons/no-avatar-user-img.png';
import { CellValue, TableRow } from '@/ui/Table/Table.interfaces';
import { SubscribeModal } from '../SubscribeModal';

function UsersTable({ data, columns, searchParams, sortedColumns }: UsersTableProps) {
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
      />
      {isModalOpen && <SubscribeModal type="unsubscribe" speakerId={idToUnsubscribe} onClose={handleCloseModal} />}
    </div>
  );
}

export { UsersTable };

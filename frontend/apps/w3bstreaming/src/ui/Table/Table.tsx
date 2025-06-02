import { ChangeEvent, useCallback, useEffect, useState } from 'react';

import arrowDownSVG from '@/assets/icons/arrow-filled-down-icon.svg';
import { cx } from '@/utils';

import { Button } from '../Button';
import { Pagination } from '../Pagination';
import { Search } from '../Search';

import {
  SortOrder,
  TableBodyProps,
  TableCellProps,
  TableHeaderCellProps,
  TableHeaderProps,
  TableProps,
  TableRow,
  TableRowProps,
} from './Table.interfaces';
import styles from './Table.module.scss';

function Cell({ className, children }: TableCellProps) {
  return <th className={cx(styles.cell, className || '')}>{children}</th>;
}

function HeaderCell({ className, children }: TableHeaderCellProps) {
  return <th className={cx(styles['header-cell'], className || '')}>{children}</th>;
}

function Header({ children }: TableHeaderProps) {
  return (
    <thead className={cx(styles.head)}>
      <tr className={cx(styles.header)}>{children}</tr>
    </thead>
  );
}

function Row({ className, children }: TableRowProps) {
  return <tr className={cx(styles.row, className || '')}>{children}</tr>;
}

function Body({ children }: TableBodyProps) {
  return <tbody className={cx(styles.body)}>{children}</tbody>;
}

function Table({
  rows,
  columns,
  pagination,
  searchParams,
  sortedColumns,
  renderCell,
  renderHeaderCell,
  className,
  renderEmpty,
}: TableProps) {
  const [tableData, setTableData] = useState<TableRow[]>(rows || []);
  const [searchedValue, setSearchedValue] = useState<string>('');
  const [currentRows, setCurrentRows] = useState<TableRow[]>(rows || []);
  const [currentPage, setCurrentPage] = useState(1);
  const [sortOrder, setSortOrder] = useState<SortOrder>({});

  useEffect(() => {
    setSortOrder(
      sortedColumns?.reduce(
        (acc, item: string) => ({
          ...acc,
          [item]: 'ascending',
        }),
        {},
      ) || {},
    );
  }, [sortedColumns]);

  const handleChangeSearch = (e: ChangeEvent<HTMLInputElement>) => {
    setSearchedValue(e.target.value);
  };

  const handleCalculateCurrentRows = useCallback(() => {
    if (pagination) {
      const indexOfFirstRow = (currentPage - 1) * pagination.rowsPerPage;

      setCurrentRows(() => tableData.slice(indexOfFirstRow, indexOfFirstRow + pagination.rowsPerPage));
    }
  }, [tableData, pagination, currentPage]);

  const handleSearch = useCallback(() => {
    if (searchParams?.column) {
      setTableData(
        rows.filter((row) =>
          row[searchParams?.column || columns[0]]?.toString().toLowerCase().includes(searchedValue.toLowerCase()),
        ),
      );
    }
  }, [searchedValue, columns, rows, searchParams]);

  const handleSortData = (column: string) => {
    if (sortOrder[column] === 'ascending') {
      setTableData((prev) => [
        ...prev.sort((a: TableRow, b: TableRow) => ((a[column] || a) > (b[column] || b) ? 1 : -1)),
      ]);

      setSortOrder((prev) => ({
        ...prev,
        [column]: 'descending',
      }));
    }

    if (sortOrder[column] === 'descending') {
      setTableData((prev) => [
        ...prev.sort((a: TableRow, b: TableRow) => ((a[column] || a) < (b[column] || b) ? 1 : -1)),
      ]);

      setSortOrder((prev) => ({
        ...prev,
        [column]: 'ascending',
      }));
    }
  };

  useEffect(() => {
    handleCalculateCurrentRows();
  }, [handleCalculateCurrentRows]);

  useEffect(() => {
    handleSearch();
  }, [handleSearch]);

  useEffect(() => {
    setTableData(rows);
  }, [rows]);

  return (
    <div className={cx(styles.container)}>
      {searchParams?.column && (
        <div className={cx(styles.search)}>
          <Search value={searchedValue} onChange={handleChangeSearch} placeholder={searchParams.placeholder} />
        </div>
      )}
      <div className={cx(styles['table-wrapper'])}>
        <table className={cx(styles.table)}>
          <Header>
            {columns.map((column) => (
              <HeaderCell key={column} className={className?.headerCell}>
                <>
                  {renderHeaderCell ? renderHeaderCell(column) : column}
                  {sortedColumns?.includes(column) && (
                    <Button variant="icon" label="" icon={arrowDownSVG} onClick={() => handleSortData(column)} />
                  )}
                </>
              </HeaderCell>
            ))}
          </Header>
          <Body>
            {tableData?.length ? (
              <>
                {currentRows.map((row) => (
                  <Row key={row.id} className={className?.row?.(row)}>
                    {columns.map((column: string) => (
                      <Cell key={column} className={className?.cell}>
                        {renderCell ? renderCell(column, row[column], row) : row[column]}
                      </Cell>
                    ))}
                  </Row>
                ))}
              </>
            ) : (
              <tr className={cx(styles['empty-table'])}>
                {renderEmpty || (
                  <td>
                    <h3 className={cx(styles['empty-table-title'])}>No items</h3>
                    <span className={cx(styles['empty-table-caption'])}>You don&apos;t have any items yet ...</span>
                  </td>
                )}
              </tr>
            )}
          </Body>
        </table>
      </div>
      {pagination && (
        <Pagination
          totalRows={tableData.length}
          rowsPerPage={pagination.rowsPerPage}
          currentPage={currentPage}
          setCurrentPage={setCurrentPage}
        />
      )}
    </div>
  );
}

export { Table };

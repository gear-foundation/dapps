import { ReactElement } from 'react';

export type CellValue = string | number | null | undefined;
export interface TableRow {
  id: string | number;
  [key: string]: CellValue;
}

export interface TableProps {
  rows: TableRow[];
  columns: string[];
  sortedColumns?: string[];
  pagination?: Pagination;
  searchParams?: SearchParams;
  renderEmpty?: ReactElement;
  renderCell?: (columnName: string | number, value: CellValue, row: TableRow) => CellValue | JSX.Element;
  renderHeaderCell?: (name: string | number) => CellValue;
  className?: {
    headerCell?: string;
    cell?: string;
    row?: (row: TableRow) => string;
  };
}

export interface TableHeaderProps {
  children: JSX.Element[];
}

export interface TableBodyProps {
  children: JSX.Element;
}

export interface TableRowProps {
  className?: string;
  children: JSX.Element[];
}

export interface TableHeaderCellProps {
  className?: string;
  children: CellValue | JSX.Element;
}

export interface TableCellProps {
  className?: string;
  children: CellValue | JSX.Element;
}

export interface Pagination {
  rowsPerPage: number;
}

export interface SearchParams {
  column: string;
  placeholder?: string;
}

export type SortDirection = 'ascending' | 'descending';

export type SortOrder = Record<string, SortDirection>;

export interface PaginationProps {
  totalRows: number;
  rowsPerPage: number;
  currentPage: number;
  setCurrentPage: (page: number) => void;
}

export interface PagesMenu {
  [key: string]: { label: string; value: number };
}

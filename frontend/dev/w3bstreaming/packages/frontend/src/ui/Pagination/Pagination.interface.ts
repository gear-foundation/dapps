export interface PaginationProps {
  totalRows: number;
  rowsPerPage: number;
  currentPage: number;
  setCurrentPage: () => void;
}

export interface PagesMenu {
  [key: string]: { label: string; value: string };
}

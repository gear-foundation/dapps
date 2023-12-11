import { useCallback, useEffect, useState } from 'react';
import vectorLeft from '@/assets/icons/vector-left.svg';
import vectorRight from '@/assets/icons/vector-right.svg';
import doubleVectorLeft from '@/assets/icons/double-vector-left.svg';
import doubleVectorRight from '@/assets/icons/double-vector-right.svg';
import { Button } from '../Button';
import styles from './Pagination.module.scss';
import { cx } from '@/utils';
import { Dropdown } from '../Dropdown';
import { PagesMenu } from './Pagination.interface';

function Pagination({ totalRows, rowsPerPage, currentPage, setCurrentPage }: any) {
  const pages = Math.ceil(totalRows / rowsPerPage);
  const [pageCount, setPageCount] = useState(Math.ceil(totalRows / rowsPerPage));
  const [generatedMenu, setGeneratedMenu] = useState<PagesMenu>({});
  const [activePagesRange, setActivePagesRange] = useState<string>('');

  const handleDefineCurrentPage = useCallback(() => {
    if (pages && currentPage > pages) {
      setCurrentPage(pages);
    }

    if (!pages) {
      setCurrentPage(1);
    }

    setPageCount(pages);
  }, [pages, currentPage, setCurrentPage]);

  const handleItemClick = ({ value }: any) => {
    setCurrentPage(value);
  };

  const calculatePageRange = useCallback(
    (page: number, itemsPerPage: number, allPages: number, allRows: number) =>
      totalRows
        ? `${page * itemsPerPage + 1} - ${page !== allPages - 1 ? page * itemsPerPage + itemsPerPage : allRows}`
        : '1 - 1',
    [totalRows],
  );

  const handleGenerateDropdownMenu = useCallback(() => {
    const menu = new Array(pages).fill(0).reduce((acc, _, i) => {
      const pageNumber = i + 1;
      const range = calculatePageRange(i, rowsPerPage, pages, totalRows);

      return {
        ...acc,
        [range]: {
          label: range,
          value: pageNumber,
        },
      };
    }, {});

    setGeneratedMenu(menu);
  }, [calculatePageRange, pages, totalRows, rowsPerPage]);

  const handleUpdateCurrentPageRange = useCallback(() => {
    setActivePagesRange(calculatePageRange(currentPage - 1, rowsPerPage, pages, totalRows));
  }, [calculatePageRange, currentPage, pages, totalRows, rowsPerPage]);

  useEffect(() => {
    handleDefineCurrentPage();
  }, [handleDefineCurrentPage]);

  useEffect(() => {
    handleGenerateDropdownMenu();
  }, [handleGenerateDropdownMenu]);

  useEffect(() => {
    handleUpdateCurrentPageRange();
  }, [handleUpdateCurrentPageRange]);

  return (
    <div className={cx(styles.pagination)}>
      <div className={cx(styles['pagination-left'])}>
        <Dropdown
          label={activePagesRange}
          menu={generatedMenu}
          className={{ menu: styles['dropdown-menu'], menuItem: styles['dropdown-menu-item'] }}
          onItemClick={handleItemClick}
        />
        <span className={cx(styles['pagination-left-out-of'])}>out of {totalRows || 1}</span>
      </div>
      <div className={cx(styles['pagination-right'])}>
        <Button
          variant="icon"
          icon={doubleVectorLeft}
          label=""
          disabled={currentPage < 2}
          onClick={() => setCurrentPage(1)}
        />
        <Button
          variant="icon"
          icon={vectorLeft}
          label=""
          disabled={currentPage < 2}
          onClick={() => setCurrentPage(currentPage - 1)}
        />
        <span>{currentPage}</span>
        <span>out of</span>
        <span>{pageCount || 1}</span>
        <Button
          variant="icon"
          icon={vectorRight}
          label=""
          disabled={currentPage > pageCount - 1}
          onClick={() => setCurrentPage(currentPage + 1)}
        />
        <Button
          variant="icon"
          icon={doubleVectorRight}
          label=""
          disabled={currentPage > pageCount - 1}
          onClick={() => setCurrentPage(pageCount)}
        />
      </div>
    </div>
  );
}

export { Pagination };

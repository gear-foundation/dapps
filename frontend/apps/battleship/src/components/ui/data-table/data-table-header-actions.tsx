import { Header } from '@tanstack/table-core';
import clsx from 'clsx';
import { ArrowDownAZ, ArrowUpAZ } from 'lucide-react';
import { CaretDown } from '@/assets/images';
import styles from './table.module.scss';

type TableHeaderActionsProps<TData, TValue> = {
  header: Header<TData, TValue>;
  title: string;
};

export function DataTableHeaderActions<TData, TValue>({ header, title }: TableHeaderActionsProps<TData, TValue>) {
  return (
    <div className={styles['header-col']}>
      <button type="button" onClick={header.column.getToggleSortingHandler()} className={styles.sorting}>
        {{
          asc: <ArrowDownAZ width={16} height={16} />,
          desc: <ArrowUpAZ width={16} height={16} />,
        }[header.column.getIsSorted() as string] ?? <CaretDown width={20} height={20} />}
      </button>
      {/*<DropdownMenu>*/}
      {/*  <DropdownMenuTrigger className="transition-colors hover:text-neutral-300">*/}
      {/*    <span className="sr-only">Open</span>*/}
      {/*    <MoreVertical width={20} height={20} />*/}
      {/*  </DropdownMenuTrigger>*/}
      {/*  <DropdownMenuContent className="text-neutral-300">*/}
      {/*    /!*<DropdownMenuLabel>Sorting</DropdownMenuLabel>*!/*/}
      {/*    /!*<DropdownMenuSeparator />*!/*/}
      {/*    <DropdownMenuItem onClick={() => header.column.clearSorting()}>*/}
      {/*      <ListX className="w-4 h-4 mr-2" />*/}
      {/*      Clear sort*/}
      {/*    </DropdownMenuItem>*/}
      {/*    <DropdownMenuItem onClick={() => header.column.toggleSorting(false)}>*/}
      {/*      <ArrowDownAZ className="w-4 h-4 mr-2" />*/}
      {/*      Sort by {title} ascending*/}
      {/*    </DropdownMenuItem>*/}
      {/*    <DropdownMenuItem onClick={() => header.column.toggleSorting(true)}>*/}
      {/*      <ArrowUpAZ className="w-4 h-4 mr-2" />*/}
      {/*      Sort by {title} descending*/}
      {/*    </DropdownMenuItem>*/}
      {/*  </DropdownMenuContent>*/}
      {/*</DropdownMenu>*/}
    </div>
  );
}

import { useState } from 'react';

import searchIcon from '@/assets/icons/search-icon.svg';
import crossIcon from '@/assets/icons/cross-icon.svg';
import styles from './SearchModal.module.scss';
import { SearchModalDialog } from './SearchModalDialog';
import { Button } from '@/ui';

export function SearchModal() {
  const [open, setOpen] = useState(false);

  return (
    <div className={styles.wrapper}>
      <Button
        variant="icon"
        className={styles.toggle}
        onClick={() => setOpen(true)}
        icon={open ? crossIcon : searchIcon}
      />

      <SearchModalDialog setOpen={setOpen} open={open} />
    </div>
  );
}

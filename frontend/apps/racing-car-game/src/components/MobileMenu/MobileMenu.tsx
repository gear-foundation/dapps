import { useState } from 'react';

import burgerMenuIcon from '@/assets/icons/burger-menu-icon.svg';
import crossIcon from '@/assets/icons/cross-icon.svg';
import styles from './MobileMenu.module.scss';
import { MobileMenuDialog } from './MobileMenuDialog';
import { Button } from '@/components/ui';

export function MobileMenu() {
  const [open, setOpen] = useState(false);

  return (
    <div className={styles.wrapper}>
      <Button
        variant="icon"
        className={styles.toggle}
        onClick={() => setOpen(true)}
        icon={open ? crossIcon : burgerMenuIcon}
      />

      <MobileMenuDialog setOpen={setOpen} open={open} />
    </div>
  );
}

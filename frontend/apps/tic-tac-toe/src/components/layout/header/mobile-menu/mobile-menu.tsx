import { Button } from '@/components/ui/button';
import styles from './mobile-menu.module.scss';
import { useState } from 'react';
import { DialogsLibrary } from '@/components/ui/dialogs';
import { Sprite } from '@/components/ui/sprite';

export function MobileMenu() {
  const [open, setOpen] = useState(false);

  return (
    <div className={styles.wrapper}>
      <Button variant="white" className={styles.toggle} onClick={() => setOpen(true)}>
        <Sprite name={open ? 'close' : 'burger-menu'} width={25} height={24} />
      </Button>
      <DialogsLibrary.MobileMenuDialog setOpen={setOpen} open={open} />
    </div>
  );
}

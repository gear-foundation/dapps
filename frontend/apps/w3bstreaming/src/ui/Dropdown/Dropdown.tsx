import { useRef, useState } from 'react';

import selectArrow from '@/assets/icons/select-arrow.svg';
import { useClickOutside } from '@/hooks';
import { cx } from '@/utils';

import { DropdownMenuItem, DropdownProps } from './Dropdown.interfaces';
import styles from './Dropdown.module.scss';

function Dropdown({
  label,
  menu,
  toggleArrowSize = 'small',
  alignMenu = 'center',
  className,
  onItemClick,
}: DropdownProps) {
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const dropdownRef = useRef<HTMLButtonElement>(null);

  useClickOutside(
    () => {
      setOpen(false);
    },
    menuRef,
    dropdownRef,
  );

  const handleItemCLick = (item: DropdownMenuItem) => {
    setOpen(false);
    onItemClick(item);
  };

  return (
    <div className={cx(styles.container)}>
      <button onClick={() => setOpen(!open)} className={cx(styles.dropdown)} ref={dropdownRef}>
        <span className={cx(styles['dropdown-label'])}>{label}</span>
        <img
          src={selectArrow}
          alt="chevron"
          className={cx(
            styles['dropdown-toggle-arrow'],
            styles[`dropdown-toggle-arrow-${toggleArrowSize}`],
            open ? styles['dropdown-toggle-arrow-rotated'] : '',
          )}
        />
      </button>

      {open && (
        <div
          className={cx(
            styles['dropdown-menu'],
            styles[`dropdown-menu-align-${alignMenu}`],
            className?.menu ? className.menu : '',
          )}
          ref={menuRef}>
          <ul>
            {Object.keys(menu).map((item) => (
              <li
                key={menu[item].value}
                className={cx(styles['dropdown-menu-item'], className?.menuItem ? className.menuItem : '')}>
                <button onClick={() => handleItemCLick(menu[item])} className={cx(styles['dropdown-menu-button'])}>
                  {menu[item].label}
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

export { Dropdown };

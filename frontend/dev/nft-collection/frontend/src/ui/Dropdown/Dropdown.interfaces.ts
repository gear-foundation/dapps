interface DropdownMenu {
  [key: string]: DropdownMenuItem;
}

export interface DropdownMenuItem {
  label: string;
  value: string;
}

export interface DropdownProps {
  label?: string | JSX.Element;
  menu: DropdownMenu;
  activeValue?: string;
  defaultSelected?: string;
  toggleArrowSize?: 'small' | 'medium' | 'large';
  alignMenu?: 'left' | 'center' | 'right';
  className?: {
    menu?: string;
    menuItem?: string;
  };
  onItemClick: (key: string, item: DropdownMenuItem) => void;
}

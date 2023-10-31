interface Menu {
  [key: string]: {
    url: string;
  };
}
export interface HeaderProps {
  menu?: Menu;
}

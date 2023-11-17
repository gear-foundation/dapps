export interface BurgerMenuProps {
  burgerMenuHandler: () => void;
}

export interface BurgerMenuItemProps {
  item: Item;
  img?: string;
  active?: boolean;
  isSubMenuOpen?: boolean;
  subMenuHandler?: (isSubMenuOpen: boolean) => void;
  isAuthenticated?: boolean;
  handleLogoutUser?: () => void;
}

export interface BurgerMenuSubItemProps {
  item: SubItem;
}

export type Item = {
  label: string;
  url?: string;
  subItems?: Array<SubItem>;
};

export type SubItem = {
  label: string;
  link: string;
  img?: string;
};

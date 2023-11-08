export interface Option {
  name: string;
  value: string;
}

export interface MultiSwithProps {
  options: Option[];
  defaultSelected?: string;
  onSelectOption?: (option: Option) => void;
}

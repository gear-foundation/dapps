export interface TabPanelProps {
  tabs: TabsConfig;
  defaultTabId?: string;
}

export interface TabProps {
  name: string;
  isActive: boolean;
  onClick: () => void;
}

export interface TabConfig {
  name: string;
  component: () => React.ReactNode;
  icon?: string;
}

export interface TabsConfig {
  [tabId: string]: TabConfig;
}

import { useState } from 'react';
import styles from './TabPanel.module.scss';
import { TabPanelProps, TabProps } from './TabsPanel.interfaces';
import { cx } from '@/utils';

function Tab({ name, isActive, onClick }: TabProps) {
  const handleTabClick = () => {
    onClick();
  };

  return (
    <button type="button" onClick={handleTabClick} className={cx(styles.tab, isActive ? styles['tab-active'] : '')}>
      {name}
    </button>
  );
}

function TabPanel({ tabs, defaultTabId }: TabPanelProps) {
  const [activeTabId, setActiveTabId] = useState<string>(defaultTabId || Object.keys(tabs)[0]);

  const handleTabClick = (tabId: string) => {
    setActiveTabId(tabId);
  };

  return (
    <div className={cx(styles['tab-panel'])}>
      <div className={cx(styles.tabs)}>
        {Object.keys(tabs).map((tabId: string) => (
          <Tab
            key={tabId}
            name={tabs[tabId].name}
            isActive={activeTabId === tabId}
            onClick={() => handleTabClick(tabId)}
          />
        ))}
      </div>
      <div className={cx(styles.content)}>{tabs[activeTabId].component()}</div>
    </div>
  );
}

export { TabPanel };

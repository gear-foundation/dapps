import { useContext } from 'react';

import { AutoSignlessContext, AutoSignlessContextType } from './context';

const useAutoSignless = (): AutoSignlessContextType => {
  const context = useContext(AutoSignlessContext);

  if (!context) {
    throw new Error('useTransactionWithSessionModal must be used within AutoSignlessProvider');
  }
  return context;
};

export { useAutoSignless };

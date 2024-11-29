import { createContext, ReactNode, useContext, useState } from 'react';

export const AppCtx = createContext({} as ReturnType<typeof useProgram>);
export const useApp = () => useContext(AppCtx);

function useProgram() {
  const [isPending, setIsPending] = useState<boolean>(false);

  return { isPending, setIsPending };
}

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}

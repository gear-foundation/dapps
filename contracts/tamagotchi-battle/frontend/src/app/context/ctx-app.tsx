import { createContext, useContext, useState } from "react";

export const AppCtx = createContext({} as ReturnType<typeof useProgram>);
export const useApp = () => useContext(AppCtx);

const useProgram = () => {
  const [isPending, setIsPending] = useState<boolean>(false);
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [isDataReady, setIsDataReady] = useState<boolean>(false);

  return {
    isPending,
    setIsPending,
    isAdmin,
    setIsAdmin,
    isDataReady,
    setIsDataReady
  };
};

export function AppProvider({ children }: React.PropsWithChildren) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}

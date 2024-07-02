import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { GearApi } from '@gear-js/api';
import { ADDRESS } from '@/app/consts';
import { Program } from '@/app/utils/sails/lib/lib';

interface ProgramContextType {
  program: Program | null;
}

const ProgramContext = createContext<ProgramContextType | undefined>(undefined);

interface ProgramProviderProps {
  children: ReactNode;
}

const ProgramProvider: React.FC<ProgramProviderProps> = ({ children }) => {
  const [program, setProgram] = useState<Program | null>(null);

  useEffect(() => {
    const initSails = async () => {
      const api = await GearApi.create({ providerAddress: ADDRESS.NODE });
      const programInstance = new Program(api, ADDRESS.GAME);

      setProgram(programInstance);
    };

    initSails();
  }, []);

  return <ProgramContext.Provider value={{ program }}>{children}</ProgramContext.Provider>;
};

const useProgram = (): Program => {
  const context = useContext(ProgramContext);
  if (context === undefined) {
    throw new Error('useProgram must be used within a ProgramProvider');
  }
  return context.program!;
};

export { useProgram, ProgramProvider };

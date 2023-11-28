import { HexString } from '@polkadot/util/types';
import { useEffect, useState } from 'react';
import { useCreateSupplyChain } from 'hooks';
import { FORM, LOCAL_STORAGE } from 'consts';
import { Create } from './create';
import { Use } from './use';
import { Start } from './start';
import { Program } from './program';

const initProgramId = (localStorage[LOCAL_STORAGE.PROGRAM] ?? '') as HexString;

function Home() {
  const [form, setForm] = useState('');

  const [programId, setProgramId] = useState(initProgramId);
  const createSupplyChain = useCreateSupplyChain(setProgramId);

  const openUseForm = () => setForm(FORM.USE);
  const openCreateForm = () => setForm(FORM.CREATE);
  const closeForm = () => setForm('');
  const closeProgram = () => setProgramId('' as HexString);

  const getForm = () => {
    switch (form) {
      case FORM.CREATE:
        return <Create onCancel={closeForm} onSubmit={createSupplyChain} />;
      case FORM.USE:
        return <Use onCancel={closeForm} onSubmit={setProgramId} />;
      default:
        return <Start onCreate={openCreateForm} onUse={openUseForm} />;
    }
  };

  useEffect(() => {
    if (programId) {
      localStorage.setItem(LOCAL_STORAGE.PROGRAM, programId);
      closeForm();
    }
  }, [programId]);

  return programId ? <Program id={programId} onBackButtonClick={closeProgram} /> : getForm();
}

export { Home };

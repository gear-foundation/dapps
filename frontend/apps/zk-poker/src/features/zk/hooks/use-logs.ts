import { atom, useAtom } from 'jotai';

const logsAtom = atom<string[]>([]);

const useLogs = () => {
  const [logs, setLogs] = useAtom(logsAtom);

  return { logs, setLogs };
};

export { useLogs };

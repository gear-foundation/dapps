import { useAccount } from '@gear-js/react-hooks';
import { createContext, PropsWithChildren, useContext, useEffect, useState } from 'react';
import { LessonState } from '@/app/types/lessons';
import { useProgramMetadata } from '../hooks/use-metadata';
import meta5 from '@/assets/meta/meta5.txt';

const key = 'tmgState';

const useProgram = () => {
  const { account, isAccountReady } = useAccount();

  const [lesson, setLesson] = useState<LessonState>();
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [isReady, setIsReady] = useState<boolean>(false);

  const resetLesson = () => {
    setLesson(undefined);
    setIsAdmin(false);
    setIsReady(false);
    localStorage.removeItem(key);
  };

  const lessonMeta = useProgramMetadata(meta5);

  useEffect(() => {
    if (!isAccountReady) return;

    resetLesson();
  }, [account, isAccountReady]);

  return {
    lesson,
    setLesson,
    lessonMeta,
    // setLessonMeta,
    isAdmin,
    setIsAdmin,
    isReady,
    setIsReady,
    resetLesson,
  };
};

export const LessonsCtx = createContext({} as ReturnType<typeof useProgram>);

export const useLessons = () => useContext(LessonsCtx);

export function LessonsProvider({ children }: PropsWithChildren) {
  const { Provider } = LessonsCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}

import { useAccount } from '@gear-js/react-hooks';
import { JSX, PropsWithChildren, useCallback, useEffect, useMemo, useState } from 'react';

import meta5 from '@/assets/meta/meta5.txt';

import { useProgramMetadata } from '../hooks/use-metadata';

import { LessonsCtx, LessonsContextValue } from './ctx-lesson.context';

const STORAGE_KEY = 'tmgState';

export function LessonsProvider({ children }: PropsWithChildren): JSX.Element {
  const { account, isAccountReady } = useAccount();

  const [lesson, setLesson] = useState<LessonsContextValue['lesson']>();
  const [isAdmin, setIsAdmin] = useState(false);
  const [isReady, setIsReady] = useState(false);

  const lessonMeta = useProgramMetadata(meta5);

  const resetLesson = useCallback(() => {
    setLesson(undefined);
    setIsAdmin(false);
    setIsReady(false);
    localStorage.removeItem(STORAGE_KEY);
  }, []);

  useEffect(() => {
    if (!isAccountReady) return;

    resetLesson();
  }, [account, isAccountReady, resetLesson]);

  const value = useMemo<LessonsContextValue>(
    () => ({
      lesson,
      setLesson,
      lessonMeta,
      isAdmin,
      setIsAdmin,
      isReady,
      setIsReady,
      resetLesson,
    }),
    [isAdmin, isReady, lesson, lessonMeta, resetLesson],
  );

  return <LessonsCtx.Provider value={value}>{children}</LessonsCtx.Provider>;
}

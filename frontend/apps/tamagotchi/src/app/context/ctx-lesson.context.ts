import type { ProgramMetadata } from '@gear-js/api';
import { createContext, Dispatch, SetStateAction, useContext } from 'react';

import type { LessonState } from '@/app/types/lessons';

export type LessonsContextValue = {
  lesson: LessonState | undefined;
  setLesson: Dispatch<SetStateAction<LessonState | undefined>>;
  lessonMeta: ProgramMetadata | undefined;
  isAdmin: boolean;
  setIsAdmin: Dispatch<SetStateAction<boolean>>;
  isReady: boolean;
  setIsReady: Dispatch<SetStateAction<boolean>>;
  resetLesson: () => void;
};

export const LessonsCtx = createContext<LessonsContextValue | undefined>(undefined);

export function useLessons() {
  const context = useContext(LessonsCtx);

  if (!context) {
    throw new Error('useLessons must be used within a LessonsProvider');
  }

  return context;
}

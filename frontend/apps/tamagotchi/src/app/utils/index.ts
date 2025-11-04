import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

import { NotificationResponseTypes, NotificationType, TamagotchiState } from '@/app/types/lessons';

export const getNotificationTypeValue = (
  str: NotificationResponseTypes,
  tamagotchi?: TamagotchiState,
): NotificationType => {
  switch (str) {
    case 'FeedMe':
      return { FeedMe: tamagotchi ? tamagotchi?.fed : undefined };
    case 'PlayWithMe':
      return { PlayWithMe: tamagotchi ? tamagotchi?.entertained : undefined };
    case 'WantToSleep':
      return { WantToSleep: tamagotchi ? tamagotchi?.rested : undefined };
  }
};

export const sleep = (s: number) => new Promise((resolve) => setTimeout(resolve, s * 1000));

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

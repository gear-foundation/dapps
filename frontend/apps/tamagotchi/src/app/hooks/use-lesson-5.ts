import { useEffect, useState } from 'react';
import type { UserMessageSent } from '@gear-js/api';
import { useApi } from '@gear-js/react-hooks';
import type { UnsubscribePromise } from '@polkadot/api/types';
import { useLessons, useTamagotchi } from '../context';
import type { NotificationResponseTypes, NotificationType } from '@/app/types/lessons';
import { getNotificationTypeValue } from '@/app/utils';

export const useLesson5 = () => {
  const { api, isApiReady } = useApi();
  const [notification, setNotification] = useState<NotificationType>({});
  const [activeNotification, setActiveNotification] = useState<NotificationResponseTypes>();
  const { lesson, lessonMeta } = useLessons();
  const { tamagotchi } = useTamagotchi();

  useEffect(() => {
    if (tamagotchi) {
      if (tamagotchi.isDead) {
        setActiveNotification(undefined);
      } else {
        if (Object.keys(notification).length) {
          const minValue = Object.entries(notification)
            .filter((item) => Boolean(item[1]))
            .sort(([, v1], [, v2]) => +v1 - +v2)[0];
          if (minValue) {
            setActiveNotification(minValue[0] as NotificationResponseTypes);
          } else setActiveNotification(undefined);
        }
      }
    }
  }, [notification, tamagotchi]);

  useEffect(() => {
    if (!isApiReady) return;

    let unsub: UnsubscribePromise | undefined;

    if (lessonMeta && lesson?.step === 5 && tamagotchi) {
      if (!tamagotchi.isDead) {
        unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }: UserMessageSent) => {
          const {
            message: { payload },
          } = data;

          const decodedPayload = lessonMeta
            .createType(lessonMeta.types.handle.output!, payload)
            .toHuman() as NotificationResponseTypes;

          if (tamagotchi && ['WantToSleep', 'PlayWithMe', 'FeedMe'].includes(decodedPayload)) {
            const update = getNotificationTypeValue(decodedPayload, tamagotchi);
            setNotification((prev) => ({ ...prev, ...update }));
          }
        });
      }
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
  }, [isApiReady, lesson, lessonMeta, tamagotchi]);

  return {
    notification,
    setNotification,
    activeNotification,
    setActiveNotification,
  };
};

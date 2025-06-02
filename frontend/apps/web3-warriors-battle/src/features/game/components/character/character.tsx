import clsx from 'clsx';
import React, { Suspense, memo } from 'react';

import { Appearance } from '@/app/utils';
import { Text } from '@/components';

import { BackColor, BodyColor, LoaderIcon } from '../../assets/images';
import { getLazySvg } from '../../utils';

import styles from './character.module.scss';

export type CharacterView = Appearance;

type FixedLengthArray<T, L extends number> = [T, ...T[]] & { length: L };
export type AppearanceIdentifiers = FixedLengthArray<number, 6>;

type CharacterProps = CharacterView & {
  fallback?: React.ReactNode;
  withSpiner?: boolean;
  loaderBackground?: boolean;
  size?: 'md' | 'sm';
};

export const Character = memo(
  (props: CharacterProps) => {
    const {
      accessory_index,
      body_index,
      hat_index,
      head_index,
      body_color,
      back_color,
      fallback,
      withSpiner = true,
      loaderBackground = false,
      size = 'md',
    } = props;

    const Hat = getLazySvg('hat', hat_index);
    const Head = getLazySvg('head', head_index);
    const Body = getLazySvg('body', body_index);
    const Accessory = getLazySvg('accessories', accessory_index);

    return (
      <div className={clsx(styles.container, size === 'sm' && styles.sm)}>
        <Suspense
          fallback={
            <>
              {fallback && <div className={styles.fallback}>{fallback}</div>}
              {withSpiner && (
                <div className={clsx(styles.loader, loaderBackground && styles.loaderBackground)}>
                  <LoaderIcon />
                  <Text size="xs" weight="semibold" className={styles.loaderText}>
                    Please wait
                  </Text>
                </div>
              )}
            </>
          }>
          <BackColor style={{ color: back_color }} />
          <BodyColor style={{ color: body_color }} />
          <Head />
          <Hat />
          <Body />
          <Accessory />
        </Suspense>
      </div>
    );
  },
  (prev, next) =>
    prev.accessory_index === next.accessory_index &&
    prev.back_color === next.back_color &&
    prev.body_color === next.body_color &&
    prev.body_index === next.body_index &&
    prev.hat_index === next.hat_index &&
    prev.head_index === next.head_index,
);

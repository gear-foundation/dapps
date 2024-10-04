import clsx from 'clsx';
import React, { Suspense, memo } from 'react';
import { BackColor, BodyColor, LoaderIcon } from '../../assets/images';
import { getLazySvg } from '../../utils';
import { Text } from '@/components';
import styles from './character.module.scss';

export type CharacterView = {
  headIndex: number;
  hatIndex: number;
  bodyIndex: number;
  accessoryIndex: number;
  bodyColor: string;
  backColor: string;
};

type CharacterProps = CharacterView & {
  fallback?: React.ReactNode;
  withSpiner?: boolean;
  loaderBackground?: boolean;
};

export const Character = memo((props: CharacterProps) => {
  const {
    accessoryIndex,
    bodyIndex,
    hatIndex,
    headIndex,
    bodyColor,
    backColor,
    fallback,
    withSpiner = true,
    loaderBackground = false,
  } = props;

  const Hat = getLazySvg('hat', hatIndex);
  const Head = getLazySvg('head', headIndex);
  const Body = getLazySvg('body', bodyIndex);
  const Accessory = getLazySvg('accessories', accessoryIndex);

  return (
    <div className={styles.container}>
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
        <BackColor style={{ color: backColor }} />
        <BodyColor style={{ color: bodyColor }} />
        <Head />
        <Hat />
        <Body />
        <Accessory />
      </Suspense>
    </div>
  );
});

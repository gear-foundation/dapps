import React, { Suspense } from 'react';
import { BackColor, BodyColor, LoaderIcon } from '../../assets/images';
import { getLazySvg } from '../../utils';
import styles from './character.module.scss';
import { Text } from '@/components';

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
};

export const Character = (props: CharacterProps) => {
  const { accessoryIndex, bodyIndex, hatIndex, headIndex, bodyColor, backColor, fallback, withSpiner = true } = props;

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
              <div className={styles.loader}>
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
};

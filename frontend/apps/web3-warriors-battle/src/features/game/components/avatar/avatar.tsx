import { VariantProps } from 'class-variance-authority';
import { Suspense, memo } from 'react';

import { BodyColor } from '../../assets/images';
import { getLazySvg } from '../../utils';
import { CharacterView } from '../character/character';

import styles from './avatar.module.scss';
import { avatarVariants } from './avatar.variants';

type AvatarProps = VariantProps<typeof avatarVariants> & CharacterView;

export const Avatar = memo((props: AvatarProps) => {
  const { size, hat_index, head_index, body_color } = props;

  const Hat = getLazySvg('hat', hat_index);
  const Head = getLazySvg('head', head_index);

  return (
    <div className={avatarVariants({ className: styles.container, size })}>
      <Suspense fallback={null}>
        <div className={styles.wrapper}>
          <BodyColor style={{ color: body_color }} />
          <Head />
          <Hat />
        </div>
      </Suspense>
    </div>
  );
});

Avatar.displayName = 'Avatar';

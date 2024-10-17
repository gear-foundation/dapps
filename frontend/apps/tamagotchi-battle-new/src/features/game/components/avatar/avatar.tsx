import styles from './avatar.module.scss';
import { VariantProps, cva } from 'class-variance-authority';
import { CharacterView } from '../character/character';
import { getLazySvg } from '../../utils';
import { Suspense, memo } from 'react';
import { BodyColor } from '../../assets/images';

export const variants = cva('', {
  variants: { size: { md: styles.md, sm: styles.sm } },
  defaultVariants: { size: 'md' },
});

type AvatarProps = VariantProps<typeof variants> & CharacterView;

export const Avatar = memo((props: AvatarProps) => {
  const { size, hat_index, head_index, body_color } = props;

  const Hat = getLazySvg('hat', hat_index);
  const Head = getLazySvg('head', head_index);

  return (
    <div className={variants({ className: styles.container, size })}>
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

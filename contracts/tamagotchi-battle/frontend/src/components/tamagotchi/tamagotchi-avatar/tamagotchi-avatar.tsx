import { useEffect, useRef, useState } from 'react';
import clsx from 'clsx';
import { Icon } from 'components/ui/icon';
import { StoreItemsNames } from 'app/types/ft-store';
import { getTamagotchiAgeDiff } from 'app/utils/get-tamagotchi-age';
import { TamagotchiAvatarAge, TamagotchiAvatarEmotions } from 'app/types/tamagotchi';
import { TamagotchiColor } from '../../../app/types/battles';
import { getTamagotchiColor } from '../../../app/utils/get-tamagotchi-color';

type TamagotchiAvatarProps = {
  emotion?: TamagotchiAvatarEmotions;
  age?: TamagotchiAvatarAge;
  isDead?: boolean;
  hasItem?: StoreItemsNames[];
  color?: TamagotchiColor;
  className?: string;
  isActive?: boolean;
  inBattle?: boolean;
  isWinner?: boolean;
  energy?: number;
};

export const TamagotchiAvatar = ({
  className,
  emotion = 'happy',
  age = 'baby',
  isDead,
  hasItem = [],
  color = 'Green',
  isActive,
  isWinner,
  energy,
  inBattle,
}: TamagotchiAvatarProps) => {
  const [dead, setDead] = useState<boolean>(Boolean(isDead));
  const [currentEmotion, setCurrentEmotion] = useState<TamagotchiAvatarEmotions>(emotion);
  const [damage, setDamage] = useState<number>(0);
  const [itemsUsed, setItemsUsed] = useState<StoreItemsNames[]>(hasItem);
  const info = useRef({ isReady: false, energy: 0 });
  const [tamagotchiAge, setTamagotchiAge] = useState<TamagotchiAvatarAge>(age);

  useEffect(() => {
    if (energy && !isActive) {
      if (info.current.isReady) {
        if (info.current.energy !== energy) {
          setDamage(Math.round((energy - info.current.energy) / 100));
          info.current.energy = energy;
        }
      } else {
        info.current.isReady = true;
        info.current.energy = energy;
      }
    } else setDamage(0);
  }, [energy, isActive]);

  const s = 'tamagotchi';
  const cn = 'absolute inset-0 w-full h-full';
  const tamagotchiDied = isDead || dead;
  const emo: TamagotchiAvatarEmotions = tamagotchiDied ? 'scared' : isWinner ? 'hello' : currentEmotion;

  const mouse = tamagotchiAge === 'baby' ? 'face-baby' : `mouse-${tamagotchiAge}-${emo === 'hello' ? 'happy' : emo}`;
  const head = `head-${tamagotchiAge}`;
  const eye = `eye-${emo === 'hello' ? 'happy' : emo}`;
  const hands = `hands-${
    itemsUsed?.includes('sword') ? 'sword' : emo === 'hello' ? 'hello' : emo === 'angry' ? 'angry' : 'normal'
  }`;
  const tail = `tail-${itemsUsed?.includes('sword') ? 'sword' : emo === 'hello' ? 'hello' : 'normal'}`;
  const glasses = itemsUsed?.includes('glasses') ? 'head-glasses' : tamagotchiAge === 'old' ? 'face-old-glasses' : null;
  const body = `body-${tamagotchiDied ? 'dead' : 'normal'}`;

  return (
    <div className={clsx('relative', getTamagotchiColor(color).body, className ?? 'grow w-full h-30 aspect-square')}>
      <svg
        width="530"
        height="614"
        viewBox="0 0 530 614"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className={clsx(
          'absolute inset-x-0 top-1/2 w-full h-auto aspect-[450/523] -translate-y-1/2 transition-opacity duration-1000',
          !isActive && 'opacity-0',
        )}>
        <g filter="url(#filter0_f_61_21481)">
          <ellipse cx="265" cy="513.5" rx="225" ry="60.5" fill="#A6A6A6" />
        </g>
        <g style={{ mixBlendMode: 'color-dodge' }} opacity="0.45" filter="url(#filter1_f_61_21481)">
          <path
            d="M77.9948 495.748C69.6856 527.441 93.5956 558.428 126.36 558.428H396.715C429.215 558.428 453.078 527.908 445.239 496.368L334.546 50.9987H194.596L77.9948 495.748Z"
            fill="url(#paint0_linear_61_21481)"
          />
        </g>
        <defs>
          <filter
            id="filter0_f_61_21481"
            x="0"
            y="413"
            width="530"
            height="201"
            filterUnits="userSpaceOnUse"
            colorInterpolationFilters="sRGB">
            <feFlood floodOpacity="0" result="BackgroundImageFix" />
            <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
            <feGaussianBlur stdDeviation="20" result="effect1_foregroundBlur_61_21481" />
          </filter>
          <filter
            id="filter1_f_61_21481"
            x="26.3335"
            y="0.998779"
            width="470.406"
            height="607.429"
            filterUnits="userSpaceOnUse"
            colorInterpolationFilters="sRGB">
            <feFlood floodOpacity="0" result="BackgroundImageFix" />
            <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
            <feGaussianBlur stdDeviation="25" result="effect1_foregroundBlur_61_21481" />
          </filter>
          <linearGradient
            id="paint0_linear_61_21481"
            x1="245.857"
            y1="558.428"
            x2="245.857"
            y2="50.9987"
            gradientUnits="userSpaceOnUse">
            <stop stopColor="#CECECE" stopOpacity="0" />
            <stop offset="0.350975" stopColor="#CBCBCB" />
            <stop offset="1" stopColor="#BCBCBC" />
          </linearGradient>
        </defs>
      </svg>
      {!tamagotchiDied && <Icon name={tail} section={s} className={cn} />}
      {!tamagotchiDied && <Icon name={hands} section={s} className={cn} />}
      <Icon name="body-stand" section={s} className={cn} />
      <Icon name="sneakers" section={s} className={clsx(cn, getTamagotchiColor(color).sneakers)} />
      <Icon name={body} section={s} className={cn} />
      {itemsUsed?.includes('bag') && <Icon name="body-bag" section={s} className={cn} />}
      <Icon name={head} section={s} className={cn} />
      <Icon name={mouse} section={s} className={cn} />
      <Icon name={eye} section={s} className={clsx(cn, 'text-[#16B768]')} />
      {emo === 'crying' && <Icon name="tears" section={s} className={cn} />}
      {!tamagotchiDied && glasses && <Icon name={glasses} section={s} className={cn} />}
      {!tamagotchiDied && itemsUsed?.includes('hat') && <Icon name="head-hat" section={s} className={cn} />}
      {Boolean(damage) && (
        <div className="absolute top-1/4 right-15 w-12 h-12 grid place-items-center">
          <Icon name="damage" section={s} className="absolute inset-0 w-full h-full" />
          <span className="relative z-1 text-white font-bold">{damage}</span>
        </div>
      )}
    </div>
  );
};

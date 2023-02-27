import clsx from 'clsx';
import { Icon } from 'components/ui/icon';
import { StoreItemsNames } from 'app/types/ft-store';
import { getTamagotchiAgeDiff } from 'app/utils/get-tamagotchi-age';
import { TamagotchiAvatarEmotions } from 'app/types/tamagotchi';
import { BattleRoundMoveVariants, TamagotchiColor } from 'app/types/battles';
import { getTamagotchiColor } from 'app/utils/get-tamagotchi-color';
import { AnimatePresence, motion } from 'framer-motion';

const transition = {
  duration: 1,
  delay: 0.5,
  ease: [0, 0.71, 0.2, 1.01],
};

type TamagotchiAvatarProps = {
  emotion?: TamagotchiAvatarEmotions;
  age?: number;
  isDead?: boolean;
  hasItem?: StoreItemsNames[];
  color?: TamagotchiColor;
  className?: string;
  isActive?: boolean;
  isWinner?: boolean;
  damage?: number;
  action?: BattleRoundMoveVariants;
  reverse?: boolean;
  asPlayer?: boolean;
};

const variants = {
  enter: { opacity: 0, y: 50 },
  center: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: 50 },
};

export const TamagotchiAvatar = ({
  className,
  emotion = 'happy',
  age = 0,
  isDead,
  hasItem = [],
  color = 'Green',
  isActive,
  isWinner,
  damage,
  action,
  reverse,
  asPlayer,
}: TamagotchiAvatarProps) => {
  const tamagotchiAge = getTamagotchiAgeDiff(age);

  const s = 'tamagotchi';
  const t = 'max-w-full w-full h-full';
  const cn = `absolute inset-0 ${t}`;
  const maxH = `max-h-[calc(100vh_-_12px_-_80px_-_60px_-_40px_-_188px_-_16px_-_144px_-_20px)]`;
  const emo: TamagotchiAvatarEmotions = isDead ? 'scared' : isWinner ? 'hello' : emotion;
  const mouse = tamagotchiAge === 'baby' ? 'face-baby' : `mouse-${tamagotchiAge}-${emo === 'hello' ? 'happy' : emo}`;
  const head = `head-${tamagotchiAge}`;
  const eye = `eye-${emo === 'hello' ? 'happy' : emo}`;
  const hands = `hands-${
    hasItem?.includes('sword') ? 'sword' : emo === 'hello' ? 'hello' : emo === 'angry' ? 'angry' : 'normal'
  }`;
  const tail = `tail-${hasItem?.includes('sword') ? 'sword' : emo === 'hello' ? 'hello' : 'normal'}`;
  const glasses = hasItem?.includes('glasses') ? 'head-glasses' : tamagotchiAge === 'old' ? 'face-old-glasses' : null;
  const body = `body-${isDead ? 'dead' : 'normal'}`;

  return (
    <AnimatePresence initial={false}>
      <div className={clsx('relative', getTamagotchiColor(color).body, className)}>
        <TamagotchiAvatarActiveScene isActive={Boolean(isActive)} />
        <TamagotchiAvatarWinnerScene isActive={Boolean(isWinner)} />

        {isDead && asPlayer && (
          <div className="absolute inset-x-0 top-1/2 -translate-y-1/2 flex justify-center items-center h-full">
            <div className="w-full">
              <div className={clsx('mx-auto aspect-square flex justify-center items-end', maxH)}>
                <Icon name="dead-shadow" section={s} className="w-auto h-5 mb-[3.72%] animate-deadTamagotchiShadow" />
              </div>
            </div>
          </div>
        )}

        <div
          className={clsx(
            'relative flex flex-col h-full max-h-full',
            maxH,
            isDead && asPlayer ? 'animate-deadTamagotchi' : 'animate-tBreath',
          )}>
          <div className="relative flex flex-col my-auto h-full">
            <div className="relative aspect-square h-full max-w-full mx-auto">
              {!isDead && <Icon name={tail} section={s} className={cn} />}
              {!isDead && <Icon name={hands} section={s} className={cn} />}
              {!isDead && <Icon name="body-stand" section={s} className={cn} />}
              {!isDead && <Icon name="sneakers" section={s} className={clsx(cn, getTamagotchiColor(color).sneakers)} />}
              <Icon name={body} section={s} className={cn} />
              {hasItem?.includes('bag') && <Icon name="body-bag" section={s} className={cn} />}
              <Icon name={head} section={s} className={cn} />
              <Icon name={mouse} section={s} className={clsx('relative', t)} />
              <Icon name={eye} section={s} className={clsx(cn, 'text-[#16B768]')} />
              {emo === 'crying' && <Icon name="tears" section={s} className={cn} />}
              {!isDead && glasses && <Icon name={glasses} section={s} className={cn} />}
              {!isDead && hasItem?.includes('hat') && <Icon name="head-hat" section={s} className={cn} />}

              {isDead && asPlayer && (
                <motion.div
                  key="death"
                  variants={variants}
                  initial={{ opacity: 0, y: 50 }}
                  animate="center"
                  exit="exit"
                  transition={transition}
                  className={clsx(
                    'absolute bottom-[70%] w-10 xxl:w-12 aspect-square grid place-items-center pointer-events-none',
                    reverse ? 'right-[8%]' : 'left-[8%]',
                    'animate-deadTamagotchiIcon',
                  )}>
                  <Icon
                    name="damage"
                    section={s}
                    className={clsx('absolute inset-0 w-full h-full', !reverse && '-scale-x-100')}
                  />
                  <Icon name="death" section={s} className="relative z-1 w-[45%] aspect-square text-white" />
                </motion.div>
              )}
            </div>
          </div>
        </div>

        {!isDead && !isWinner && !!damage && (
          <motion.div
            key="damage"
            variants={variants}
            initial="enter"
            animate="center"
            exit="exit"
            transition={{ ...transition }}
            className={clsx(
              'absolute top-1/4 w-12 h-12 grid place-items-center pointer-events-none',
              reverse ? 'right-[10%]' : 'left-[10%]',
            )}>
            <Icon
              name="damage"
              section={s}
              className={clsx('absolute inset-0 w-full h-full', !reverse && '-scale-x-100')}
            />
            <span className="relative z-1 text-white font-bold">-{damage}</span>
          </motion.div>
        )}
        {!isDead && !isWinner && action && (
          <motion.div
            key="action"
            variants={variants}
            initial="enter"
            animate="center"
            exit="exit"
            transition={{ ...transition, delay: 1 }}
            className="absolute -top-4 inset-0 h-fit leading-4 text-center text-white"
            aria-hidden>
            <span
              className={clsx(
                'inline-flex py-0.5 px-4 font-bold rounded-full',
                action === 'Defence' ? 'bg-theme-blue' : 'bg-tertiary',
              )}>
              {action}
            </span>
          </motion.div>
        )}
      </div>
    </AnimatePresence>
  );
};

const TamagotchiAvatarWinnerScene = ({ isActive }: { isActive: boolean }) => (
  <div
    className={clsx(
      'absolute -top-[12%] -inset-x-[8%] -bottom-[16.5%] transition-opacity duration-1000 opacity-100',
      !isActive && '!opacity-0',
    )}>
    <svg
      className="w-full h-full overflow-visible"
      width="550"
      height="586"
      viewBox="0 0 550 586"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <g opacity="0.7" filter="url(#filter0_f_160_17759)">
        <ellipse cx="275" cy="497.5" rx="225" ry="60.5" fill="#22C43D" />
      </g>
      <g className="" filter="url(#filter1_f_160_17759)">
        <path
          d="M91.7511 509.897C83.5554 541.555 107.454 572.429 140.155 572.429H405.909C438.348 572.429 462.2 542.016 454.468 510.511L345.126 64.9997H206.93L91.7511 509.897Z"
          fill="url(#paint0_linear_160_17759)"
        />
      </g>
      <defs>
        <filter
          id="filter0_f_160_17759"
          x="0"
          y="387"
          width="550"
          height="221"
          filterUnits="userSpaceOnUse"
          colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix" />
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
          <feGaussianBlur stdDeviation="25" result="effect1_foregroundBlur_160_17759" />
        </filter>
        <filter
          id="filter1_f_160_17759"
          x="25.1294"
          y="0"
          width="495.803"
          height="637.429"
          filterUnits="userSpaceOnUse"
          colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix" />
          <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
          <feGaussianBlur stdDeviation="32.5" result="effect1_foregroundBlur_160_17759" />
        </filter>
        <linearGradient
          id="paint0_linear_160_17759"
          x1="257.549"
          y1="572.429"
          x2="257.549"
          y2="64.9997"
          gradientUnits="userSpaceOnUse">
          <stop stopColor="#16B768" stopOpacity="0" />
          <stop offset="0.350975" stopColor="#16B768" />
          <stop offset="1" stopColor="#16B768" />
        </linearGradient>
      </defs>
    </svg>

    {/*<svg*/}
    {/*  className="w-full h-full"*/}
    {/*  width="450"*/}
    {/*  height="508"*/}
    {/*  viewBox="0 0 450 508"*/}
    {/*  fill="none"*/}
    {/*  xmlns="http://www.w3.org/2000/svg">*/}
    {/*  <ellipse className="blur-lg" opacity="0.7" cx="225" cy="432.5" rx="225" ry="60.5" fill="#22C43D" />*/}
    {/*  <g className="blur-[32px]">*/}
    {/*    <path*/}
    {/*      d="M41.7511 444.897C33.5554 476.555 57.4543 507.429 90.1553 507.429H355.909C388.348 507.429 412.2 477.016 404.468 445.511L295.126 -0.000279844H156.93L41.7511 444.897Z"*/}
    {/*      fill="url(#paint0_linear_153_3424)"*/}
    {/*    />*/}
    {/*  </g>*/}
    {/*  <defs>*/}
    {/*    <linearGradient*/}
    {/*      id="paint0_linear_153_3424"*/}
    {/*      x1="207.549"*/}
    {/*      y1="507.429"*/}
    {/*      x2="207.549"*/}
    {/*      y2="-0.000274658"*/}
    {/*      gradientUnits="userSpaceOnUse">*/}
    {/*      <stop stopColor="#16B768" stopOpacity="0" />*/}
    {/*      <stop offset="0.350975" stopColor="#16B768" />*/}
    {/*      <stop offset="1" stopColor="#16B768" />*/}
    {/*    </linearGradient>*/}
    {/*  </defs>*/}
    {/*</svg>*/}
  </div>
);
const TamagotchiAvatarActiveScene = ({ isActive }: { isActive: boolean }) => {
  return (
    <svg
      width="530"
      height="614"
      viewBox="0 0 530 614"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={clsx(
        'absolute left-1/2 top-1/2 -z-1 w-auto h-[120%] aspect-[450/523] -translate-y-1/2 -translate-x-1/2 transition-opacity duration-1000',
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
  );
};

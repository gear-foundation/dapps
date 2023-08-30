import { AnimatePresence, motion } from 'framer-motion'
import { SpriteIcon } from '@/components/ui/sprite-icon'
import { getTamagotchiAgeDiff, getTamagotchiColor } from '../../utils'
import type { StoreItemsNames } from '../../types/ft-store'
import type { TamagotchiAvatarEmotions } from '../../types/tamagotchi'
import type {
  BattleRoundMoveVariants,
  TamagotchiColor,
} from '../../types/battles'
import { cn } from '@/app/utils'

const transition = {
  duration: 0.5,
  delay: 1,
}

type TamagotchiAvatarProps = {
  emotion?: TamagotchiAvatarEmotions
  age?: number
  isDead?: boolean
  hasItem?: StoreItemsNames[]
  color?: TamagotchiColor
  className?: string
  isActive?: boolean
  isWinner?: boolean
  damage?: number
  action?: BattleRoundMoveVariants
  reverse?: boolean
  asPlayer?: boolean
}

const variants = {
  enter: { opacity: 0, y: 50 },
  center: { opacity: 1, y: 0 },
  exit: { opacity: 0, transition: { delay: 0 } },
}

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
  const tamagotchiAge = getTamagotchiAgeDiff(age)

  const s = 'tamagotchi'
  const t = 'max-w-full w-full h-full'
  const cx = `absolute inset-0 ${t}`
  const maxH = `max-h-[calc(100vh_-_12px_-_80px_-_60px_-_40px_-_140px_-_16px_-_132px_-_20px)]`
  const emo: TamagotchiAvatarEmotions = isDead
    ? 'scared'
    : isWinner
    ? 'hello'
    : emotion
  const mouse =
    tamagotchiAge === 'baby'
      ? 'face-baby'
      : `mouse-${tamagotchiAge}-${emo === 'hello' ? 'happy' : emo}`
  const head = `head-${tamagotchiAge}`
  const eye = `eye-${emo === 'hello' ? 'happy' : emo}`
  const hands = `hands-${
    hasItem?.includes('sword')
      ? 'sword'
      : emo === 'hello'
      ? 'hello'
      : emo === 'angry'
      ? 'angry'
      : 'normal'
  }`
  const tail = `tail-${
    hasItem?.includes('sword') ? 'sword' : emo === 'hello' ? 'hello' : 'normal'
  }`
  const glasses = hasItem?.includes('glasses')
    ? 'head-glasses'
    : tamagotchiAge === 'old'
    ? 'face-old-glasses'
    : null
  const body = `body-${isDead ? 'dead' : 'normal'}`

  const showScene = isActive || isWinner
  const showAction = !isDead && !isWinner && action
  const showDamage = !isDead && !isWinner && !!damage

  return (
    <>
      <div
        className={cn(
          'relative flex flex-col',
          getTamagotchiColor(color).body,
          className
        )}
      >
        <AnimatePresence key="ap-scene">
          {showScene && (
            <motion.div
              key="tamagotchi-backdrop-scene"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0, transition: { delay: 0, duration: 1 } }}
              transition={{ duration: 0.5, delay: 1 }}
              className={cn(
                'absolute -top-[12%] -inset-x-[8%] -bottom-[16.5%] -z-1'
              )}
            >
              <BackdropScene isWinner={Boolean(isWinner)} />
            </motion.div>
          )}
        </AnimatePresence>

        <div className="absolute inset-x-0 top-1/2 -translate-y-1/2 flex justify-center items-center h-full">
          <div className="w-full">
            <div
              className={cn(
                'relative mx-auto aspect-square flex justify-center items-end',
                maxH
              )}
            >
              <AnimatePresence key="ap-damages">
                {isDead && asPlayer && (
                  <SpriteIcon
                    name="dead-shadow"
                    section={s}
                    className="max-w-[30%] h-5 mb-[3.72%] animate-deadTamagotchiShadow"
                  />
                )}

                {showDamage && (
                  <motion.div
                    key="damage"
                    variants={variants}
                    initial="enter"
                    animate="center"
                    exit="exit"
                    transition={{ ...transition }}
                    className={cn(
                      'absolute top-1/4 smh:w-9 w-12 aspect-square pointer-events-none',
                      reverse
                        ? 'smh:right-[5%] right-[10%]'
                        : 'smh:left-[5%] left-[10%]'
                    )}
                  >
                    <div className="grid place-items-center w-full h-full animate-damageIcon">
                      <SpriteIcon
                        name="damage"
                        section={s}
                        className={cn(
                          'absolute inset-0 w-full h-full',
                          !reverse && '-scale-x-100'
                        )}
                      />
                      <span className="relative z-1 text-white font-bold smh:text-xs text-sm">
                        -{damage}
                      </span>
                    </div>
                  </motion.div>
                )}
                {showAction && (
                  <motion.div
                    key="action"
                    variants={variants}
                    initial="enter"
                    animate="center"
                    exit="exit"
                    transition={{ ...transition, delay: 1.2 }}
                    className="absolute -top-4 inset-0 h-fit leading-4 text-center text-white"
                    aria-hidden
                  >
                    <p
                      className={cn(
                        'inline-flex py-1 px-4 font-bold rounded-full',
                        action === 'Defence' && 'bg-theme-blue',
                        action === 'Attack' && 'bg-tertiary',
                        action === 'Skipped' && 'bg-white/20'
                      )}
                    >
                      {action}
                    </p>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          </div>
        </div>

        <motion.div
          key="tamagotchi-avatar"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0, transition: { delay: 0 } }}
          transition={{ duration: 0.5, delay: 0.5 }}
          className={cn(
            'absolute inset-0 grow',
            maxH,
            isDead && asPlayer
              ? 'animate-deadTamagotchi'
              : reverse
              ? 'animate-tBreath2'
              : 'animate-tBreath'
          )}
        >
          {!isDead && <SpriteIcon name={tail} section={s} className={cx} />}
          {!isDead && <SpriteIcon name={hands} section={s} className={cx} />}
          {!isDead && (
            <SpriteIcon name="body-stand" section={s} className={cx} />
          )}
          {!isDead && (
            <SpriteIcon
              name="sneakers"
              section={s}
              className={cn(cx, getTamagotchiColor(color).sneakers)}
            />
          )}
          <SpriteIcon name={body} section={s} className={cx} />
          {hasItem?.includes('bag') && (
            <SpriteIcon name="body-bag" section={s} className={cx} />
          )}
          <SpriteIcon name={head} section={s} className={cx} />
          <SpriteIcon name={mouse} section={s} className={cx} />
          <SpriteIcon
            name={eye}
            section={s}
            className={cn(cx, 'text-[#16B768]')}
          />
          {emo === 'crying' && (
            <SpriteIcon name="tears" section={s} className={cx} />
          )}
          {!isDead && glasses && (
            <SpriteIcon name={glasses} section={s} className={cx} />
          )}
          {!isDead && hasItem?.includes('hat') && (
            <SpriteIcon name="head-hat" section={s} className={cx} />
          )}

          {isDead && asPlayer && (
            <motion.div
              key="death"
              variants={variants}
              initial={{ opacity: 0, y: 50 }}
              animate="center"
              exit="exit"
              transition={transition}
              className={cn(
                'absolute bottom-[70%] w-10 xxl:w-12 aspect-square grid place-items-center pointer-events-none',
                reverse ? 'right-[8%]' : 'left-[8%]',
                'animate-deadTamagotchiIcon'
              )}
            >
              <SpriteIcon
                name="damage"
                section={s}
                className={cn(
                  'absolute inset-0 w-full h-full',
                  !reverse && '-scale-x-100'
                )}
              />
              <SpriteIcon
                name="death"
                section={s}
                className="relative z-1 w-[45%] aspect-square text-white"
              />
            </motion.div>
          )}
        </motion.div>
      </div>
    </>
  )
}

const BackdropScene = ({ isWinner }: { isWinner: boolean }) => (
  <svg
    className="w-full h-full overflow-visible before:absolute before:-inset-10 before:-z-1 before:bg-[#1e1e1e]"
    width="450"
    height="508"
    viewBox="0 0 450 508"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <ellipse
      className={isWinner ? 'blur-lg' : 'blur-[20px]'}
      opacity={isWinner ? 0.7 : 1}
      cx="225"
      cy="432.5"
      rx="225"
      ry="60.5"
      fill={isWinner ? '#22c43d' : '#a6a6a6'}
    />
    <g
      className={cn(
        'mix-blend-color-dodge',
        isWinner ? 'blur-[32px]' : 'blur-[25px]'
      )}
      opacity={isWinner ? 1 : 0.45}
    >
      <path
        d="M41.7511 444.897C33.5554 476.555 57.4543 507.429 90.1553 507.429H355.909C388.348 507.429 412.2 477.016 404.468 445.511L295.126 -0.000279844H156.93L41.7511 444.897Z"
        fill={`url(#${isWinner ? '__winner' : '__active'})`}
      />
    </g>
    <defs>
      <linearGradient
        id="__winner"
        x1="207.549"
        y1="507.429"
        x2="207.549"
        y2="-0.000274658"
        gradientUnits="userSpaceOnUse"
      >
        <stop stopColor="#16B768" stopOpacity="0" />
        <stop offset="0.350975" stopColor="#16B768" />
        <stop offset="1" stopColor="#16B768" />
      </linearGradient>
      <linearGradient
        id="__active"
        x1="207.549"
        y1="507.429"
        x2="207.549"
        y2="-0.000274658"
        gradientUnits="userSpaceOnUse"
      >
        <stop stopColor="#CECECE" stopOpacity="0" />
        <stop offset="0.350975" stopColor="#CBCBCB" />
        <stop offset="1" stopColor="#BCBCBC" />
      </linearGradient>
    </defs>
  </svg>
)

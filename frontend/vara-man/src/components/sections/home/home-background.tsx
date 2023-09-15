import IntroSmokeImage from '@/assets/images/intro-smoke.webp'
import IntroMapImage from '@/assets/images/intro-map.webp'

type HomeBackgroundProps = BaseComponentProps & {}


export function HomeBackground({ }: HomeBackgroundProps) {
  return (
    <div className="absolute inset-0 flex justify-center h-full bg-[#1E1E1E] pointer-events-none">
      <div className="absolute top-15 -bottom-0 -inset-x-[31.5%] z-1 mix-blend-color-dodge">
        <img
          src={IntroSmokeImage}
          alt="aa"
          className="w-full h-full object-contain"
          loading="lazy"
        />
      </div>
      <div className="relative mb-[-13%]">
        <img
          src={IntroMapImage}
          alt="aa"
          className="w-full h-full"
          loading="lazy"
        />
      </div>
    </div>
  )
}

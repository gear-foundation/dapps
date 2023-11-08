import { HomeBackground } from '@/components/sections/home/home-background'
import { HomeFormGradient } from '@/components/sections/home/home-form-gradient'
import IntroClockImage from '@/assets/images/intro-clock.png'

export function HomeNotActive() {
  return (
    <>
      <HomeBackground grayscale />
      <div className="flex justify-center items-center grow h-full">
        <div className="relative w-full max-w-[650px] ">
          <div className="absolute inset-x-0 top-0 -mx-[5px] mt-[-5px] bg-[#1F1F1F]/30 backdrop-blur-[5px] rounded-t-[17px] grayscale">
            <HomeFormGradient />
          </div>

          <div className="relative z-1 grid gap-8 w-full max-w-[650px] pt-13 pb-12 px-5 bg-[#1F1F1F]/30 backdrop-blur-[5px] rounded-t-[17px]">
            <div className="relative ml-auto mr-auto">
              <img
                src={IntroClockImage}
                alt=""
                className="w-[100px] grayscale"
                loading="lazy"
              />
            </div>
            <h1 className="typo-h2 text-center">
              The application is <span className="text-[#F24A4A]">temporarily unavailable</span>
            </h1>
            <p className="text-center text-sm">Follow the news in our <a href='https://discord.com/invite/7BQznC9uD9' target='_blank' rel="noreferrer" className="underline">Discord</a> and  <a href='https://t.me/VaraNetwork_Global' target='_blank' rel="noreferrer" className="underline">Telegram</a>.<br />
              In the meantime, visit our <a href='https://wiki.gear-tech.io/docs/examples/prerequisites' target='_blank' rel="noreferrer" className="underline">Wiki</a> page.
            </p>

          </div>
        </div>
      </div>
    </>
  )
}

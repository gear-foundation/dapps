import { useAccount } from '@gear-js/react-hooks'
import { CreateTamagotchiForm } from '@/components/forms/create-tamagotchi-form'
import { LoginSection } from '@/components/sections/login-section'

export const ConnectAccount = () => {
  const { account } = useAccount()

  return (
    <div className="flex flex-col items-center gap-9 text-center w-full">
      <div className="space-y-6">
        {account ? (
          <>
            <h2 className="typo-h2 text-primary">Geary</h2>
            <p className="text-[#D1D1D1]">
              Insert program ID to create a character
            </p>
          </>
        ) : (
          <h2 className="typo-h2 text-center">
            <span className="block text-primary">Connect your account</span> to
            start the game
          </h2>
        )}
      </div>
      <div className=" w-full">
        {account ? <CreateTamagotchiForm /> : <LoginSection />}
      </div>
    </div>
  )
}

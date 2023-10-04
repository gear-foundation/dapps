import { Button, Input } from '@gear-js/ui'
import { useForm } from '@mantine/form'
import { hexRequired } from '@/app/utils'
import { useBattle } from '../../context'
import { useBattleMessage } from '../../hooks'
import { HexString } from '@polkadot/util/types'

const createTamagotchiInitial = {
  hero_id: '' as HexString,
}

const validate: Record<string, typeof hexRequired> = {
  hero_id: hexRequired,
}

export const CreateTamagotchiForm = () => {
  const { battle, isPending } = useBattle()
  const handleMessage = useBattleMessage()
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  })
  const { getInputProps, errors } = form
  const handleSubmit = form.onSubmit(({ hero_id }) => {
    handleMessage(
      { Register: { hero_id } },
      {
        onSuccess: () => form.reset(),
      }
    )
  })

  return (
    <form onSubmit={handleSubmit} className="flex items-start gap-6">
      <div className="basis-[420px]">
        <Input
          placeholder="Insert program ID"
          direction="y"
          {...getInputProps('hero_id')}
        />
      </div>
      <div className="whitespace-nowrap">
        <Button
          text="Register"
          color="primary"
          type="submit"
          disabled={
            Object.keys(errors).length > 0 ||
            isPending ||
            battle?.status !== 'Registration'
          }
        />
      </div>
    </form>
  )
}

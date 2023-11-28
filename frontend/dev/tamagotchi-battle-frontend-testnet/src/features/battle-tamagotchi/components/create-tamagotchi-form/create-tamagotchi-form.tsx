import { Button, Input } from '@gear-js/ui'
import { useForm } from '@mantine/form'
import { hexRequired } from '@/app/utils'
import { useBattle } from '../../context'
import { useBattleMessage } from '../../hooks'
import { useNavigate } from 'react-router-dom'
import { HexString } from '@polkadot/util/types'

const createTamagotchiInitial = {
  programId: '' as HexString,
  programId2: '' as HexString,
  currentStep: 1,
}

const validate: Record<string, typeof hexRequired> = {
  programId: hexRequired,
}

export const CreateTamagotchiForm = () => {
  const { battle, isPending } = useBattle()
  const handleMessage = useBattleMessage()
  const navigate = useNavigate()
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  })
  const { getInputProps, errors } = form
  const handleSubmit = form.onSubmit((values) => {
    handleMessage(
      { Register: { tmg_id: values.programId } },
      {
        onSuccess: () => {
          form.reset()
          navigate('/battle')
        },
        onError: () => form.reset(),
      }
    )
  })

  return (
    <form onSubmit={handleSubmit} className="flex items-start gap-6">
      <div className="basis-[420px]">
        <Input
          placeholder="Insert program ID"
          direction="y"
          {...getInputProps('programId')}
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
            battle?.state !== 'Registration'
          }
        />
      </div>
    </form>
  )
}

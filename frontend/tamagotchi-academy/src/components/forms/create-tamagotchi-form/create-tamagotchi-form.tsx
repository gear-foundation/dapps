import { Button, buttonStyles, Input, Select } from '@gear-js/ui'
import { useForm } from '@mantine/form'
import { hexRequired } from '@/app/utils/form-validations'
import { useApp, useLessons } from '@/app/context'
import { createTamagotchiInitial } from '@/app/consts'
import { Link } from 'react-router-dom'
import { cn } from '@/app/utils'

const validate: Record<string, typeof hexRequired> = {
  programId: hexRequired,
}

const options = [
  { value: 1, label: 'Lesson 1' },
  { value: 2, label: 'Lesson 2' },
  { value: 3, label: 'Lesson 3' },
  { value: 4, label: 'Lesson 4' },
  { value: 5, label: 'Lesson 5' },
  { value: 6, label: 'Lesson 6' },
]

export const CreateTamagotchiForm = () => {
  const { isPending } = useApp()
  const { setLesson } = useLessons()
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  })
  const { getInputProps, errors } = form
  const handleSubmit = form.onSubmit((values) => {
    setLesson({ step: +values.currentStep, programId: values.programId })
  })

  return (
    <form
      onSubmit={handleSubmit}
      className="flex items-start justify-center gap-6"
    >
      {+form.values.currentStep === 6 ? (
        <Link
          to="/battle"
          className={cn('btn gap-2 whitespace-nowrap', buttonStyles.primary)}
        >
          Let's Battle!
        </Link>
      ) : (
        <>
          <div className="basis-[400px]">
            <Input
              placeholder="Insert program ID"
              direction="y"
              {...getInputProps('programId')}
            />
          </div>
          <div className="">
            <Select
              options={options}
              direction="y"
              {...getInputProps('currentStep')}
            />
          </div>
          <div className="whitespace-nowrap">
            <Button
              text="Create Tamagochi"
              color="primary"
              type="submit"
              disabled={Object.keys(errors).length > 0 || isPending}
            />
          </div>
        </>
      )}
    </form>
  )
}

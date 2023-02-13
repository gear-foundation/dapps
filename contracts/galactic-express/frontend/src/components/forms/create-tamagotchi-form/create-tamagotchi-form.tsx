import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { hexRequired } from 'app/utils/form-validations';
import { createTamagotchiInitial } from 'app/consts';
import { useApp } from 'app/context';
import { useBattleMessage } from 'app/hooks/use-battle';
import { useNavigate } from 'react-router-dom';

const validate: Record<string, typeof hexRequired> = {
  programId: hexRequired,
};

export const CreateTamagotchiForm = () => {
  const { isPending } = useApp();
  const handleMessage = useBattleMessage();
  const navigate = useNavigate();
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors } = form;
  const handleSubmit = form.onSubmit((values) => {
    handleMessage(
      { Register: { tmg_id: values.programId } },
      {
        onSuccess: () => {
          form.reset();
          navigate('/battle');
        },
        onError: () => form.reset(),
      },
    );
  });

  return (
    <form onSubmit={handleSubmit} className="flex items-start justify-center gap-6">
      <div className="basis-[400px]">
        <Input placeholder="Insert program ID" direction="y" {...getInputProps('programId')} />
      </div>
      <div className="whitespace-nowrap">
        <Button
          text="Create Tamagotchi"
          color="primary"
          type="submit"
          disabled={Object.keys(errors).length > 0 || isPending}
        />
      </div>
    </form>
  );
};

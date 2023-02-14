import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { numberRequired } from 'app/utils/form-validations';
import { createLauncheInitial } from 'app/consts';
import { useApp } from 'app/context';
import { useBattleMessage } from 'app/hooks/use-battle';
import { useNavigate } from 'react-router-dom';

const validate: Record<string, typeof numberRequired> = {
  payload: numberRequired,
  fuel: numberRequired
};

export const LaunchRocketForm = () => {
  const { isPending } = useApp();
  const handleMessage = useBattleMessage();
  const navigate = useNavigate();
  const form = useForm({
    initialValues: createLauncheInitial,
    validate: validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors } = form;
  const handleSubmit = form.onSubmit((values) => {
    console.log(values)
    navigate('/battle');
    // handleMessage(
    //   { Register: { tmg_id: values.programId } },
    //   {
    //     onSuccess: () => {
    //       form.reset();
    //       navigate('/battle');
    //     },
    //     onError: () => form.reset(),
    //   },
    // );
  });

  return (
    <form onSubmit={handleSubmit} className="flex items-start justify-center gap-6">
      <div className="basis-[400px]">
        <Input placeholder="Payload" direction="y" {...getInputProps('payload')} />
      </div>
      <div className="basis-[400px]">
        <Input placeholder="Fuel" direction="y" {...getInputProps('fuel')} />
      </div>
      <div className="whitespace-nowrap">
        <Button
          text="Launche Rocket"
          color="primary"
          type="submit"
          className='rk-btn'
          disabled={Object.keys(errors).length > 0 || isPending}
        />
      </div>
    </form>
  );
};

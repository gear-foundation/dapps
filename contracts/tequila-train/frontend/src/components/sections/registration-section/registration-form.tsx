import { Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { stringRequired } from '../../../app/utils';
import { useGameMessage } from '../../../app/hooks/use-game';
import { useAccount } from '@gear-js/react-hooks';
import { useApp } from '../../../app/context';
import clsx from 'clsx';

const initialValues = {
  name: '',
};

const validate: Record<string, typeof stringRequired> = {
  name: stringRequired,
};

export function RegistrationForm() {
  const { setIsPending, isPending } = useApp();
  const { account } = useAccount();
  const form = useForm({
    initialValues,
    validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors, reset } = form;

  const handleMessage = useGameMessage();
  const onSuccess = () => {
    setIsPending(false);
    reset();
  };
  const onError = () => {
    setIsPending(false);
  };

  const handleSubmit = form.onSubmit((values) => {
    setIsPending(true);
    handleMessage({ Register: { player: account?.decodedAddress, name: values.name } }, { onSuccess, onError });
  });

  return (
    <form className="grid gap-6 lg:gap-0 lg:flex lg:space-x-6" onSubmit={handleSubmit}>
      <div className="text-sm grow">
        <Input
          label="Enter your name"
          placeholder="Señor Amarillo"
          className="[&_label]:text-sm [&_label]:font-normal"
          autoComplete="name"
          {...getInputProps('name')}
        />
      </div>
      <div className="">
        <button
          type="submit"
          className={clsx('btn btn--primary gap-2 tracking-[0.08em]', isPending && 'btn--loading')}
          disabled={Object.keys(errors).length > 0 || isPending}>
          Register
        </button>
      </div>
    </form>
  );
}

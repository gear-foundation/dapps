import { withoutCommas } from '@gear-js/react-hooks';
import { Button, Select } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { ReactComponent as check } from 'assets/images/icons/check.svg';
import { InfoText } from 'components';
import { Wallet } from 'types';

type Props = {
  wallets: Wallet[] | undefined;
  onSubmit: (value: string | undefined) => void;
};

function SelectWallet({ wallets, onSubmit }: Props) {
  // TODO: take a look after gear-js/ui update for undefined select value

  const initialValues = { id: withoutCommas(wallets?.[0]?.[0] || '') };
  const isAnyWallet = !!wallets?.length;

  const form = useForm({ initialValues });
  const { getInputProps } = form;

  const handleSubmit = ({ id }: typeof initialValues) => onSubmit(id);

  // TODO: walletId should be number
  const getOptions = () =>
    wallets?.map(([id]) => ({ label: id, value: withoutCommas(id) }));
  const options = getOptions() || [];

  return isAnyWallet ? (
    <form onSubmit={form.onSubmit(handleSubmit)}>
      <Select
        label="Wallet ID"
        options={options}
        color="light"
        direction="y"
        {...getInputProps('id')}
      />
      <Button type="submit" text="Continue" icon={check} block />
    </form>
  ) : (
    <InfoText text="There are no wallets in your contract, please create one" />
  );
}

export { SelectWallet };

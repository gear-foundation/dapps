import { Button, Checkbox, Modal, Select } from '@gear-js/ui';
import { useForm as useMantineForm } from '@mantine/form';
import { UseFormInput } from '@mantine/form/lib/use-form';
import { ChangeEvent } from 'react';
import styles from './PurchaseSubscriptionModal.module.scss';

const periods = [
  { label: 'Year', value: 'Year' },
  { label: '9 months', value: 'NineMonths' },
  { label: '6 months', value: 'SixMonths' },
  { label: '3 months', value: 'ThreeMonths' },
  { label: '1 month', value: 'Month' },
];

const initialValues = { isRenewal: true, period: periods[0].value };

type Props = { close: () => void; onSubmit: (values: typeof initialValues) => void };

const useForm = (input: UseFormInput<Record<string, unknown>>) => {
  const form = useMantineForm(input);
  const { values, getInputProps, setFieldValue } = form;

  const getCheckboxProps = (name: string) => getInputProps(name, { type: 'checkbox' });

  const handleRadioChange = ({ target: { name, value } }: ChangeEvent<HTMLInputElement>) => {
    setFieldValue(name, value);
  };

  const getRadioProps = (name: keyof typeof values, value: string) => {
    const checked = values[name] === value;
    const onChange = handleRadioChange;

    return { onChange, value, checked, name };
  };

  return { ...form, getCheckboxProps, getRadioProps };
};

function PurchaseSubscriptionModal({ close, onSubmit }: Props) {
  const form = useForm({ initialValues });
  const { getInputProps, getCheckboxProps } = form;

  return (
    <Modal heading="Purchase subscription" close={close}>
      {/* @ts-ignore */}
      <form className={styles.form} onSubmit={form.onSubmit(onSubmit)}>
        <Select label="Period" direction="y" options={periods} {...getInputProps('period')} />
        <Checkbox label="Enable auto-renewal" {...getCheckboxProps('isRenewal')} />
        <p className={styles.text}>
          By confirming your subscription, you hereby authorize VaraTube Inc. to charge your wallet for the amount of
          tokens for this and future payments.
        </p>
        <Button type="submit" text="Purchase subscription" />
      </form>
    </Modal>
  );
}

export { PurchaseSubscriptionModal };

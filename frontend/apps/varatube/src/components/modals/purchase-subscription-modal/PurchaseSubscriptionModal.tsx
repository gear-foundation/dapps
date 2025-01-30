import { Button, Checkbox, Modal, Select } from '@gear-js/ui';
import { useForm as useMantineForm } from '@mantine/form';
import { UseFormInput } from '@mantine/form/lib/use-form';
import { ChangeEvent } from 'react';
import styles from './PurchaseSubscriptionModal.module.scss';
import { initialValues, periods } from '@/consts';
import { FormValues } from '@/types';

type Props = { disabledSubmitButton: boolean; close: () => void; onSubmit: (values: FormValues) => void };

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

function PurchaseSubscriptionModal({ disabledSubmitButton, close, onSubmit }: Props) {
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
        <Button type="submit" text="Purchase subscription" disabled={disabledSubmitButton} />
      </form>
    </Modal>
  );
}

export { PurchaseSubscriptionModal };

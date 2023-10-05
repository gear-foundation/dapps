import { ChangeEvent, FormEvent, useEffect, useState } from 'react';
import { Button, Input } from '@gear-js/ui';
import { cx } from 'utils';
import { useAtomValue, useSetAtom } from 'jotai';
import { CURRENT_CONTRACT_ADDRESS_ATOM, IS_CONTRACT_ADDRESS_INITIALIZED_ATOM } from 'atoms';
import { useNewSessionMessage } from 'features/session/hooks';
import styles from './EnterContractAddress.module.scss';

export interface ContractFormValues {
  [key: string]: string;
}

type Props = {
  doesSessionExist: boolean;
  isUserAdmin: boolean;
  isStateComing: boolean;
};

function EnterContractAddress({ doesSessionExist, isUserAdmin, isStateComing }: Props) {
  const isContractAddressInitialized = useAtomValue(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const contractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const { meta, message: sendNewSessionMessage } = useNewSessionMessage(contractAddress);
  const setCurrentContractAddress = useSetAtom(CURRENT_CONTRACT_ADDRESS_ATOM);
  const setIsContractAddressInitialized = useSetAtom(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const [formValues, setFormValues] = useState<ContractFormValues>({
    address: '',
  });

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    setFormValues((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setCurrentContractAddress(formValues.address);
  };

  useEffect(() => {
    if (!isContractAddressInitialized && contractAddress && isStateComing) {
      if (!doesSessionExist && meta) {
        sendNewSessionMessage(
          { CreateNewSession: null },
          {
            onSuccess: () => {
              setIsContractAddressInitialized(true);
            },
            onError: () => {
              console.log('error');
            },
          },
        );
      }
      if (doesSessionExist) {
        setIsContractAddressInitialized(true);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    contractAddress,
    doesSessionExist,
    isUserAdmin,
    meta,
    // sendNewSessionMessage,
    setIsContractAddressInitialized,
    isContractAddressInitialized,
    isStateComing,
  ]);

  return (
    <div className={cx(styles.container)}>
      <form onSubmit={handleSubmit} className={cx(styles.form)}>
        <Input
          label="Contract address:"
          name="address"
          value={formValues.address}
          onChange={handleChange}
          className={cx(styles.input)}
          required
        />
        <Button type="submit" text="Continue" className={styles.button} size="large" disabled={!formValues.address} />
      </form>
    </div>
  );
}

export { EnterContractAddress };

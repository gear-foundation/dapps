import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { hexRequired } from 'app/utils';
import { useBattle } from '../../context';
import { useNavigate } from 'react-router-dom';
import { HexString } from '@polkadot/util/types';
import { useAccount } from '@gear-js/react-hooks';
import { useFetchVoucher } from 'app/hooks/use-fetch-voucher';
import { useCheckBalance } from 'features/wallet/hooks';
import { GAS_LIMIT } from 'app/consts';
import { useBattleMessage2 } from 'features/battle/hooks/use-battle';

const createTamagotchiInitial = {
  programId: '' as HexString,
  programId2: '' as HexString,
  currentStep: 1,
};

const validate: Record<string, typeof hexRequired> = {
  programId: hexRequired,
};

export const CreateTamagotchiForm = () => {
  const { battle, isPending } = useBattle();
  const handleMessage = useBattleMessage2();
  const { account } = useAccount();
  const { isVoucher, isLoading, updateBalance } = useFetchVoucher(account?.address);
  const { checkBalance } = useCheckBalance(isVoucher);
  const navigate = useNavigate();
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors } = form;

  const handleSubmit = form.onSubmit(async (values) => {
    const payload = { Register: { tmg_id: values.programId } };

    const onSuccess = () => {
      form.reset();
      navigate('/battle');
    };

    const onError = () => form.reset();

    await updateBalance();

    checkBalance(
      GAS_LIMIT,
      () => {
        handleMessage({ payload, onSuccess, onError, withVoucher: isVoucher });
      },
      onError,
    );
  });

  return (
    <>
      <form onSubmit={handleSubmit} className="flex items-start justify-center gap-6">
        <Input placeholder="Insert program ID" direction="y" {...getInputProps('programId')} />

        <div className="whitespace-nowrap">
          <Button
            text="Create Tamagotchi"
            color="primary"
            type="submit"
            disabled={Object.keys(errors).length > 0 || isPending || battle?.state !== 'Registration' || isLoading}
          />
        </div>
      </form>
      {battle?.state !== 'Registration' && (
        <div>The battle has already started. Registration is not available now.</div>
      )}
    </>
  );
};

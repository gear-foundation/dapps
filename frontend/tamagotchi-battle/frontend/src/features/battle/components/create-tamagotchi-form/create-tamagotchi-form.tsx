import { Button, Input } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { hexRequired } from 'app/utils';
import { useProgramMetadata } from 'app/hooks/api';
import { BATTLE_ADDRESS } from 'features/battle/consts';
import { useBattle } from '../../context';
import { useBattleMessage } from '../../hooks';
import { useNavigate } from 'react-router-dom';
import { HexString } from '@polkadot/util/types';
import { useHandleCalculateGas, withoutCommas } from '@gear-js/react-hooks';
import metaTxt from '../../assets/meta/battle.meta.txt';

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
  const handleMessage = useBattleMessage();
  const meta = useProgramMetadata(metaTxt);
  const calculateGas = useHandleCalculateGas(BATTLE_ADDRESS, meta);
  const navigate = useNavigate();
  const form = useForm({
    initialValues: createTamagotchiInitial,
    validate: validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors } = form;
  const handleSubmit = form.onSubmit((values) => {
    const payload = { Register: { tmg_id: values.programId } };

    // calculateGas(payload)
    //   .then((res) => res.toHuman())
    //   .then(({ min_limit }) => {
    //     const limit = withoutCommas(min_limit as string);

    //     handleMessage({
    //       payload,
    //       gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
    //       onSuccess: () => {
    //         form.reset();
    //         navigate('/battle');
    //       },
    //       onError: () => form.reset(),
    //     });
    //   })
    //   .catch(() => {
    //     alert('Gas calculation error');
    //   });

    handleMessage({
      payload,
      onSuccess: () => {
        form.reset();
        navigate('/battle');
      },
      onError: () => form.reset(),
    });
  });
  console.log(isPending);
  console.log(battle?.state);
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
          disabled={Object.keys(errors).length > 0 || isPending || battle?.state !== 'Registration'}
        />
      </div>
    </form>
  );
};

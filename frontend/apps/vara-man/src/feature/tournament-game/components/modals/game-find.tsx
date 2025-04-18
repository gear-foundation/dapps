import { useApi } from '@gear-js/react-hooks';
import { Input, Button } from '@gear-js/vara-ui';
import { hasLength, useForm } from '@mantine/form';
import { useEzTransactions } from 'gear-ez-transactions';

import { useApp } from '@/app/context/ctx-app';
import { useRegisterForTournamentMessage } from '@/app/utils';
import { Modal } from '@/components/ui/modal/modal2';
import { SpriteIcon } from '@/components/ui/sprite-icon';

type GameFindModalProps = {
  findGame: {
    admin: string;
    bid: bigint;
    participants: number;
  };
  setIsOpenFindModal: (_: boolean) => void;
};

const initialValues = {
  username: '',
};

const validate = {
  username: hasLength({ min: 2, max: 25 }, 'Username must be 2-25 characters long'),
};

export const GameFindModal = ({ findGame, setIsOpenFindModal }: GameFindModalProps) => {
  const form = useForm({
    initialValues,
    validate,
    validateInputOnChange: true,
  });
  const { getInputProps } = form;

  const { api } = useApi();
  const { isPending, setIsPending } = useApp();
  const { registerForTournamentMessage } = useRegisterForTournamentMessage();
  const { gasless } = useEzTransactions();

  const onSuccess = () => {
    setIsPending(false);
  };
  const onError = () => {
    setIsPending(false);
  };

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = Number(findGame?.bid || 0) / 10 ** decimals;

  const handleSubmit = form.onSubmit((values) => {
    setIsPending(true);

    if (!gasless.isLoading) {
      registerForTournamentMessage(BigInt(bid * 10 ** decimals), findGame.admin, values.username, {
        onSuccess,
        onError,
      });
    }
  });

  return (
    <Modal open>
      <Modal.Content classNameContent="max-w-[650px]">
        <h2 className="text-[34px]/[37px] font-semibold md:typo-h2"> The game has been found</h2>
        <div className="flex flex-col gap-5 mt-5">
          <p className="text-[#555756]">
            To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
            need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
            end of the game, any unused gas will be refunded.
          </p>

          <div className="bg-[#f0f2f3] rounded-2xl text-black p-4">
            <div className="flex flex-col gap-2">
              <div className="flex items-center justify-between pr-[100px]">
                <p>Entry fee</p>
                <div className="font-semibold flex items-center">
                  <SpriteIcon name="vara-coin" width={24} height={24} className="mr-2" />
                  {bid} VARA
                </div>
              </div>

              <div className="flex items-center justify-between pr-[100px]">
                <p>Players already joined the game</p>
                <div className="font-semibold flex items-center">
                  <SpriteIcon name="user" width={24} height={24} className="mr-2" />
                  <span className="font-semibold">{findGame.participants} </span>
                  /10
                </div>
              </div>
            </div>
          </div>

          <form onSubmit={handleSubmit}>
            <Input
              type="text"
              label="Enter your name:"
              placeholder="Username"
              required
              className="w-full"
              {...getInputProps('username')}
            />

            <div className="flex gap-10 mt-5">
              <Button color="grey" text="Cancel" className="w-full" onClick={() => setIsOpenFindModal(false)} />
              <Button type="submit" text="Join" className="w-full" isLoading={isPending} />
            </div>
          </form>
        </div>
      </Modal.Content>
    </Modal>
  );
};

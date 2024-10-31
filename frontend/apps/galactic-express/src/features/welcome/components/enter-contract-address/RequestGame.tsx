import { useEffect, useState } from 'react';
import { Wallet } from '@dapps-frontend/ui';
import { Button } from '@gear-js/vara-ui';
import { cx } from 'utils';
import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useSetAtom, useAtom } from 'jotai';
import { CURRENT_GAME_ATOM, IS_LOADING, PLAYER_NAME_ATOM, REGISTRATION_STATUS } from 'atoms';
import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { TextField } from 'components/layout/TextField';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString, decodeAddress } from '@gear-js/api';
import { GameFoundModal } from 'features/session/components/game-found-modal';
import { JoinModalFormValues } from 'features/session/components/game-found-modal/GameFoundModal';
import { TextModal } from 'features/session/components/game-not-found-modal';
import { GameIntro } from '../game-intro';
import { GameState, useGetGameQuery, useCreateNewSessionMessage } from 'app/utils';
import styles from './RequestGame.module.scss';

type Status = 'creating' | 'joining' | null;

type CreateFormValues = {
  fee: number;
  name: string;
};

type JoinFormValues = {
  address: HexString | undefined;
};

function RequestGame() {
  const { account } = useAccount();
  const { api } = useApi();
  const { getFormattedBalanceValue, getChainBalanceValue } = useBalanceFormat();

  const { createNewSessionMessage } = useCreateNewSessionMessage();

  const [foundState, setFoundState] = useState<GameState | null>(null);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);
  const setPlayerName = useSetAtom(PLAYER_NAME_ATOM);
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);
  const [status, setStatus] = useState<Status>(null);
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const existentialDeposit = Number(getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toFixed());
  const [isJoinSessionModalShown, setIsJoinSessionModalShown] = useState<boolean>(false);
  const [foundGame, setFoundGame] = useState<HexString | undefined>(undefined);
  const [gameNotFoundModal, setGameNotFoundModal] = useState<boolean>(false);

  const createForm = useForm({
    initialValues: {
      fee: existentialDeposit + 5 || 0,
      name: '',
    },
    validate: {
      fee: (value) =>
        Number(value) < existentialDeposit + 5 ? `value must be more than ${existentialDeposit + 5}` : null,
      name: isNotEmpty(`Name shouldn't be empty`),
    },
  });

  const joinForm = useForm<JoinFormValues>({
    initialValues: {
      address: undefined,
    },
    validate: {
      address: isNotEmpty(`Address shouldn't be empty`),
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit, values } = joinForm;

  const { refetch } = useGetGameQuery(values.address?.length === 49 ? decodeAddress(values.address) : undefined);

  const handleSetStatus = (newStatus: Status) => {
    setStatus(newStatus);
  };

  const handleCloseFoundModal = () => {
    setIsJoinSessionModalShown(false);
  };

  const handleCreateSession = (values: CreateFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }
    setIsLoading(true);
    createNewSessionMessage(
      { name: values.name, value: BigInt(getChainBalanceValue(values.fee).toFixed()) },
      { onSuccess: () => setIsLoading(false), onError: () => setIsLoading(false) },
    );
  };

  const handleOpenJoinSessionModal = async (values: JoinFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }

    try {
      const { data } = await refetch();

      if (data) {
        setFoundState(data);
        setFoundGame(decodeAddress(values.address || ''));
        setIsJoinSessionModalShown(true);
        return;
      }
      setGameNotFoundModal(true);
    } catch (err: any) {
      console.log(err.message);
      setGameNotFoundModal(true);
    }
  };

  const handleJoinSession = (values: JoinModalFormValues) => {
    if (foundGame) {
      setCurrentGame(foundGame);
      setPlayerName(values.name);
    }
  };

  const handleCloseNotFoundModal = () => {
    setGameNotFoundModal(false);
  };

  useEffect(() => {
    setRegistrationStatus('registration');

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className={cx(styles.container)}>
      <GameIntro status={status} />
      {account ? (
        <>
          {!status && (
            <div className={cx(styles.form)}>
              <Button
                type="submit"
                text="Find game"
                className={styles.button}
                onClick={() => handleSetStatus('joining')}
                disabled={isLoading}
              />
              <Button
                type="submit"
                text="Create game"
                color="dark"
                className={styles.button}
                onClick={() => handleSetStatus('creating')}
                disabled={isLoading}
              />
            </div>
          )}
          {status === 'creating' && (
            <form className={styles.createForm} onSubmit={onCreateSubmit(handleCreateSession)}>
              <div className={cx(styles.input)}>
                <TextField
                  label="Entry fee"
                  variant="active"
                  type="number"
                  icon={api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />}
                  disabled={isLoading}
                  {...getCreateInputProps('fee')}
                />
                <span className={cx(styles['field-error'])}>{createErrors.fee}</span>
              </div>
              <div className={cx(styles.input)}>
                <TextField
                  label="Enter your name"
                  variant="active"
                  placeholder="Your name"
                  maxLength={20}
                  disabled={isLoading}
                  {...getCreateInputProps('name')}
                />
                <span className={cx(styles['field-error'])}>{createErrors.name}</span>
              </div>
              <div className={cx(styles.form)}>
                <Button type="submit" text="Continue" disabled={isLoading} className={styles.button} />
                <Button
                  type="submit"
                  text="Cancel"
                  color="dark"
                  disabled={isLoading}
                  className={styles.button}
                  onClick={() => handleSetStatus(null)}
                />
              </div>
            </form>
          )}
          {status === 'joining' && (
            <form className={styles.createForm} onSubmit={onJoinSubmit(handleOpenJoinSessionModal)}>
              <div className={cx(styles.input)}>
                <TextField
                  label="Specify the game admin address:"
                  variant="active"
                  placeholder="kG25c..."
                  {...getJoinInputProps('address')}
                />
                <span className={cx(styles['field-error'])}>{joinErrors.address}</span>
              </div>
              <div className={cx(styles.form)}>
                <Button type="submit" text="Continue" disabled={isLoading} className={styles.button} />
                <Button
                  type="submit"
                  text="Cancel"
                  color="dark"
                  disabled={isLoading}
                  className={styles.button}
                  onClick={() => handleSetStatus(null)}
                />
              </div>
            </form>
          )}
        </>
      ) : (
        <Wallet />
      )}
      {isJoinSessionModalShown && (
        <GameFoundModal
          entryFee={getFormattedBalanceValue(String(foundState?.bid || '')).toFixed()}
          players={
            ((foundState && 'registration' in foundState?.stage && foundState?.stage.registration?.length) || 0) + 1
          }
          gasAmount={1.121}
          onSubmit={handleJoinSession}
          onClose={handleCloseFoundModal}
        />
      )}
      {gameNotFoundModal && (
        <TextModal
          heading="Game not found"
          text="Please check the entered address. It&#39;s possible the game has been canceled or does not exist."
          onClose={handleCloseNotFoundModal}
        />
      )}
    </div>
  );
}

export { RequestGame };

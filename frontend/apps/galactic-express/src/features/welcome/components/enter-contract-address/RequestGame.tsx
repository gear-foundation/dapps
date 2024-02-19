import { useState } from 'react';
import { WalletNew as Wallet } from '@dapps-frontend/ui';
import { Button } from '@gear-js/vara-ui';
import { cx } from 'utils';
import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useSetAtom, useAtom } from 'jotai';
import { CURRENT_GAME_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from 'atoms';
import { useLaunchMessage } from 'features/session/hooks';
import metaTxt from 'assets/meta/galactic_express_meta.txt';
import { useAccount, useAccountDeriveBalancesAll, useApi, useBalanceFormat, withoutCommas } from '@gear-js/react-hooks';
import { TextField } from 'components/layout/TextField';
import { isNotEmpty, useForm } from '@mantine/form';
import { HexString } from '@gear-js/api';
import { GameFoundModal } from 'features/session/components/game-found-modal';
import { ADDRESS } from 'consts';
import { useProgramMetadata } from 'hooks';
import { LaunchState, Participant } from 'features/session/types';
import { JoinModalFormValues } from 'features/session/components/game-found-modal/GameFoundModal';
import { GameNotFoundModal } from 'features/session/components/game-not-found-modal';
import { GameIntro } from '../game-intro';
import styles from './RequestGame.module.scss';

export interface ContractFormValues {
  [key: string]: string;
}

type Props = {
  doesSessionExist: boolean;
  isUserAdmin: boolean;
  isStateComing: boolean;
  participants: Participant[];
};

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
  const balances = useAccountDeriveBalancesAll();
  const { isApiReady } = useApi();
  const { api } = useApi();
  const { getFormattedBalance, getFormattedBalanceValue, getChainBalanceValue } = useBalanceFormat();
  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;
  const [foundState, setFoundState] = useState<LaunchState | null>(null);
  const { meta: isMeta, message: sendNewSessionMessage } = useLaunchMessage();
  const meta = useProgramMetadata(metaTxt);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);
  const setPlayerName = useSetAtom(PLAYER_NAME_ATOM);
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

  const joinForm = useForm({
    initialValues: {
      address: undefined,
    },
    validate: {
      address: isNotEmpty(`Address shouldn't be empty`),
    },
  });

  const { errors: createErrors, getInputProps: getCreateInputProps, onSubmit: onCreateSubmit } = createForm;

  const {
    errors: joinErrors,
    getInputProps: getJoinInputProps,
    onSubmit: onJoinSubmit,
    setFieldError: setJoinFieldError,
  } = joinForm;

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

    const payload = {
      CreateNewSession: {
        name: values.name,
      },
    };

    setIsLoading(true);
    sendNewSessionMessage({
      payload,
      value: getChainBalanceValue(values.fee).toFixed(),
      onSuccess: () => {
        setIsLoading(false);
      },
      onError: () => {
        console.log('error');
        setIsLoading(false);
      },
    });
  };

  const handleOpenJoinSessionModal = async (values: JoinFormValues) => {
    if (!account?.decodedAddress) {
      return;
    }

    const payload = { GetGame: { playerId: values.address } };

    try {
      const res = await api?.programState.read(
        {
          programId: ADDRESS.CONTRACT,
          payload,
        },
        meta,
      );

      const state = (await res?.toHuman()) as LaunchState;

      if (state?.Game) {
        setFoundState(state);
        setFoundGame(values.address);
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
                  icon={balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />}
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
                  placeholder="0x25c"
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
          entryFee={getFormattedBalanceValue(withoutCommas(foundState?.Game.bid || '')).toFixed()}
          players={(foundState?.Game.stage.Registration?.length || 0) + 1}
          gasAmount={1.121}
          onSubmit={handleJoinSession}
          onClose={handleCloseFoundModal}
        />
      )}
      {gameNotFoundModal && <GameNotFoundModal onClose={handleCloseNotFoundModal} />}
    </div>
  );
}

export { RequestGame };

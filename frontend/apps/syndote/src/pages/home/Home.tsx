import { useEffect, useRef, useState } from 'react';
import { useAtomValue, useSetAtom } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';
import clsx from 'clsx';
import { HexString } from '@polkadot/util/types';
import { fields, INIT_PLAYERS } from 'consts';
import { PlayersByStrategyAddress, Step } from 'types';
import { Loader } from 'components';
import { Roll } from './roll';
import styles from './Home.module.scss';
import { Players } from './players/Players';
import { Button } from '@gear-js/vara-ui';
import { Cell } from './cell';
import { RequestGame } from 'pages/welcome/components/request-game';
import {
  PlayerInfoState,
  useAddGasToPlayerStrategyMessage,
  useEventGameCanceledSubscription,
  useEventStepSubscription,
  useExitGameMessage,
  useGetGameSessionQuery,
  usePlayMessage,
  useRegisterMessage,
} from 'app/utils';
import { CURRENT_GAME_ADMIN_ATOM, CURRENT_STRATEGY_ID_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from 'atoms';
import { SessionInfo } from './session-info';
import { TextModal } from './text-modal';
import { ContinueGameModal } from './continue-game-modal';
import { ReserveModal } from './reserve-modal';
import { GameFinishedModal } from './game-finished-modal';

type ModalContract = 'contractRequresGas' | 'adminReservesGas' | null;
type ModalStrategy = 'strategyRequresGas' | 'playerReservesGas' | null;
type ModalGameStatus = 'gameCancelled' | 'gameFinished' | null;

function Home() {
  const { account } = useAccount();
  const isLoading = useAtomValue(IS_LOADING);
  const playerName = useAtomValue(PLAYER_NAME_ATOM);
  const [modalContract, setModalContract] = useState<ModalContract>(null);
  const [modalStrategy, setModalStrategy] = useState<ModalStrategy>(null);
  const [modalGameStatus, setModalGameStatus] = useState<ModalGameStatus>(null);
  const adminRef = useRef<null | HexString>(null);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ADMIN_ATOM);

  const { playMessage } = usePlayMessage();
  const { registerMessage } = useRegisterMessage();
  const { exitGameMessage } = useExitGameMessage();
  const { addGasToPlayerStrategyMessage } = useAddGasToPlayerStrategyMessage();

  const { state, isFetched } = useGetGameSessionQuery();
  const strategyId = useAtomValue(CURRENT_STRATEGY_ID_ATOM);
  const [steps, setSteps] = useState<Step[]>([]);
  const [step, setStep] = useState(0);

  const { admin_id, winner, game_status, entry_fee, prize_pool } = state || {};
  const isAdmin = account?.decodedAddress === admin_id;
  const isGameStarted = steps.length > 0;
  const roll = steps[step];
  const strategyNeedsGas =
    game_status && 'waitingForGasForStrategy' in game_status ? game_status.waitingForGasForStrategy : null;
  const { properties, ownership } = roll || {};
  const playersArray = state?.players || [];

  const getPlayers = () => (isGameStarted && roll ? roll?.players : state?.players || []);

  const findPlayer = (address: string) => getPlayers().find(([newAddress]) => newAddress === address)?.[1];

  const players = playersArray.map(([address], index) => ({
    ...INIT_PLAYERS[index],
    address,
    ...(findPlayer(address) as PlayerInfoState),
  }));
  const playersByStrategyAddress = players.reduce((acc, item) => {
    return {
      ...acc,
      [item.address]: item,
    };
  }, {}) as PlayersByStrategyAddress;
  const isAnyPlayer = players.length > 0;
  const playerStrategyId = players.find((player) => player.owner_id === account?.decodedAddress)?.address;

  const register = () => {
    if (!playerName || !admin_id) {
      return;
    }

    const onSuccess = () => {
      setCurrentGame(null);
    };
    registerMessage(
      {
        value: entry_fee ? BigInt(entry_fee) : undefined,
        adminId: admin_id,
        strategyId: strategyId as HexString,
        name: playerName,
      },
      { onSuccess },
    );
  };

  const startGame = () => {
    if (!admin_id) return;
    playMessage({ adminId: admin_id });
  };

  const exitGame = () => {
    if (!admin_id) return;
    exitGameMessage(
      { adminId: admin_id },
      {
        onSuccess: () => {
          adminRef.current = null;
        },
      },
    );
  };

  const addGasToPlayerStrategy = () => {
    if (!admin_id) return;
    addGasToPlayerStrategyMessage(
      { adminId: admin_id },
      {
        onSuccess: () => {
          setModalStrategy(null);
        },
      },
    );
  };

  const continueGame = () => {
    setModalContract(null);
    startGame();
  };

  useEffect(() => {
    if (admin_id) {
      adminRef.current = admin_id as HexString;
    }
    if (isFetched && !admin_id) {
      adminRef.current = null;
    }
  }, [admin_id]);

  useEffect(() => {
    if (!game_status || !('waitingForGasForGameContract' in game_status)) {
      setModalContract(null);
      return;
    }

    if (isAdmin) {
      setModalContract('contractRequresGas');
      return;
    }

    setModalContract('adminReservesGas');
  }, [game_status, isAdmin]);

  useEffect(() => {
    if (!strategyNeedsGas) {
      setModalStrategy(null);
      return;
    }

    if (strategyNeedsGas === playerStrategyId) {
      setModalStrategy('strategyRequresGas');
      return;
    }

    setModalStrategy('playerReservesGas');
  }, [strategyNeedsGas, playerStrategyId]);

  useEffect(() => {
    if (game_status && 'finished' in game_status) {
      setModalGameStatus('gameFinished');
    }
  }, [game_status]);

  useEffect(() => {
    if (steps.length > 0) {
      setStep(steps.length - 1);
    }
  }, [steps]);

  const prevStep = () => setStep((prevValue) => (prevValue - 1 >= 0 ? prevValue - 1 : prevValue));
  const nextStep = () => setStep((prevValue) => (prevValue + 1 < steps.length ? prevValue + 1 : prevValue));
  const firstStep = () => setStep(0);
  const lastStep = () => setStep(steps.length - 1);

  useEventGameCanceledSubscription(() => {
    if (adminRef.current && adminRef.current !== account?.decodedAddress) {
      setModalGameStatus('gameCancelled');
    }

    adminRef.current = null;
    setSteps([]);
  });

  useEventStepSubscription((step) => {
    setSteps((prevSteps) => [...prevSteps, step]);
  });

  const getColor = (address: HexString) => players?.find((player) => player.address === address)?.color;

  const getFields = () =>
    fields.map(({ Image, values, type }, index) => (
      <Cell
        key={index}
        index={index}
        players={players}
        Image={Image}
        ownership={ownership}
        properties={properties}
        card={values}
        type={type}
      />
    ));

  useEffect(() => {
    if (!winner || winner.startsWith('0x00')) return;

    setSteps((prevSteps) =>
      [...prevSteps].sort(
        ({ current_step }, { current_step: anotherStep }) => Number(current_step) - Number(anotherStep),
      ),
    );
  }, [winner]);

  const handleCloseModal = () => {
    setModalGameStatus(null);
    setModalContract(null);
    setModalStrategy(null);
  };

  const entryFee = entry_fee ? String(entry_fee) : null;
  if (!isFetched) return <Loader />;

  return (
    <>
      {!!state && (
        <>
          <div className={styles.players}>{isAnyPlayer && <Players list={players} winner={winner} />}</div>

          <div className={styles.field}>
            <div className={styles.wrapper}>
              {getFields()}
              <div
                className={clsx(
                  styles.controller,
                  'registration' in state.game_status ? styles.controllerWhite : styles.controllerWithInnerBorder,
                )}>
                {isGameStarted && roll ? (
                  <Roll
                    color={getColor(roll.current_player)}
                    player={roll.current_player}
                    currentTurn={step + 1}
                    turnsAmount={steps.length}
                    onPrevClick={prevStep}
                    onNextClick={nextStep}
                    onFirstClick={firstStep}
                    onLastClick={lastStep}
                  />
                ) : (
                  <div className={clsx(styles.syndoteContainer, isAdmin && styles.syndoteContainerAdmin)}>
                    {'registration' in state.game_status && (
                      <>
                        <div className={clsx(styles.headingWrapper, styles.headingWrapperAdmin)}>
                          <h1 className={styles.heading}>Registration...</h1>
                          {players.length < 4 ? (
                            <p className={styles.subheading}>
                              {isAdmin
                                ? 'Copy the program address and send it to the players so they can join you.'
                                : `Players (${playersArray.length}/4). Waiting for other players... `}
                            </p>
                          ) : (
                            <p className={styles.subheading}>
                              {!isAdmin && `Players ${playersArray.length}/4. Waiting for admin to start game...`}
                            </p>
                          )}
                        </div>
                        {!isAdmin && (
                          <>
                            {playersArray.map((item) => item[1].owner_id).includes(account?.decodedAddress || '0x') ? (
                              <Button text="Cancel" color="grey" onClick={exitGame} />
                            ) : (
                              <Button text="Register" onClick={register} isLoading={isLoading} />
                            )}
                          </>
                        )}
                        {isAdmin && (
                          <>
                            <SessionInfo entryFee={entryFee} players={state.players} adminId={state.admin_id} />
                            {players.length === 4 && (
                              <div className={styles.mainButtons}>
                                <Button
                                  text="Start the game"
                                  onClick={startGame}
                                  isLoading={isLoading}
                                  className={styles.startGameButton}
                                />
                              </div>
                            )}
                          </>
                        )}
                      </>
                    )}
                    {'play' in state.game_status && (
                      <>
                        <div className={clsx(styles.headingWrapper, styles.headingWrapperAdmin)}>
                          <h1 className={styles.heading}>Registration...</h1>
                        </div>
                        {isAdmin && (
                          <>
                            <SessionInfo entryFee={entryFee} players={state.players} adminId={state.admin_id} />
                            <div className={styles.mainButtons}>
                              <Button text="Start the game" onClick={startGame} className={styles.startGameButton} />
                            </div>
                          </>
                        )}
                        {!isAdmin && <span className={styles.subheading}>Waiting for admin to start the game...</span>}
                      </>
                    )}
                    {'finished' in state.game_status && (
                      <>
                        {state.winner === playerStrategyId ? (
                          <p className={clsx(styles.heading, styles.headingWinner)}>You're Winner!</p>
                        ) : (
                          <p className={clsx(styles.heading, styles.headingBankrupt)}>You're Bankrupt!</p>
                        )}
                      </>
                    )}
                  </div>
                )}
              </div>
            </div>
          </div>
          {modalContract === 'contractRequresGas' && (
            <ContinueGameModal onReserve={continueGame} onClose={handleCloseModal} />
          )}
          {modalContract === 'adminReservesGas' && (
            <TextModal
              heading={`Game administrator reserves extra gas to continue the game`}
              text="Once he reserves the required amount, the game will continue."
              onClose={handleCloseModal}
            />
          )}
          {modalStrategy === 'strategyRequresGas' && (
            <ReserveModal onReserve={addGasToPlayerStrategy} onClose={handleCloseModal} />
          )}
          {modalStrategy === 'playerReservesGas' && (
            <TextModal
              heading={`Player ${playersByStrategyAddress[strategyNeedsGas || '0x']?.name} pays extra gas`}
              text="Once he reserves the required amount, the game will continue. If he fails to do so before the timer expires, he will lose and can only observe the game"
              onClose={handleCloseModal}
            />
          )}
          {modalGameStatus === 'gameFinished' && winner && (
            <GameFinishedModal
              winnerAddress={winner}
              isAdmin={isAdmin}
              prizePool={String(prize_pool)}
              players={playersByStrategyAddress}
              onClose={handleCloseModal}
            />
          )}
        </>
      )}
      {!state && (
        <div className={styles.container}>
          <div className={styles.requestGameContainer}>
            <RequestGame />
          </div>
          <div className={styles.downloadGameContainer}>
            <h4 className={styles.donwloadTitle}>Get Started</h4>
            <p className={styles.donwloadText}>
              To quickly get started, download the default Wasm program of the game, upload it to the blockchain
              network, and then copy its address to specify it in the game
            </p>
            <div className={styles.donwloadButtons}>
              <a href="https://github.com/gear-foundation/dapps/releases/download/nightly/syndote_player.opt.wasm">
                <Button color="transparent" text="Download file" />
              </a>
              <a
                target="_blank"
                href="https://wiki.gear-tech.io/docs/examples/Gaming/monopoly/#%EF%B8%8F-build-master-and-player-programs">
                <Button color="transparent" text="How does it work?" />
              </a>
            </div>
          </div>
        </div>
      )}
      {modalGameStatus === 'gameCancelled' && (
        <TextModal
          heading="The game has been canceled by the administrator"
          text="Game administrator has ended the game. All spent VARA tokens for the entry fee will be refunded."
          onClose={handleCloseModal}
        />
      )}
    </>
  );
}

export { Home };

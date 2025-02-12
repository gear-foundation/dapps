import { useAccount, useApi, withoutCommas } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { UnsubscribePromise } from '@polkadot/api/types';
import { Bytes } from '@polkadot/types';
import { HexString } from '@polkadot/util/types';
import clsx from 'clsx';
import { useAtomValue, useSetAtom, useAtom } from 'jotai';
import { useEffect, useRef, useState } from 'react';

import { useCheckBalance, useDnsProgramIds } from '@dapps-frontend/hooks';

import meta from '@/assets/meta/syndote_meta.txt';
import { CURRENT_GAME_ADMIN_ATOM, CURRENT_STRATEGY_ID_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from '@/atoms';
import { Loader } from '@/components';
import { fields, INIT_PLAYERS } from '@/consts';
import { useProgramMetadata, useReadGameSessionState, useSyndoteMessage } from '@/hooks/metadata';
import { MessageHandlePayload, MessagePayload, PlayerState, PlayersByStrategyAddress, Step } from '@/types';

import { RequestGame } from '../welcome/components/request-game';

import styles from './Home.module.scss';
import { Cell } from './cell';
import { ContinueGameModal } from './continue-game-modal';
import { GameFinishedModal } from './game-finished-modal';
import { Players } from './players/Players';
import { ReserveModal } from './reserve-modal';
import { Roll } from './roll';


import { SessionInfo } from './session-info';


import { TextModal } from './text-modal';

type ModalContract = 'contractRequresGas' | 'adminReservesGas' | null;
type ModalStrategy = 'strategyRequresGas' | 'playerReservesGas' | null;
type ModalGameStatus = 'gameCancelled' | 'gameFinished' | null;

function Home() {
  const { account } = useAccount();
  const { api } = useApi();
  const metadata = useProgramMetadata(meta);
  const { programId } = useDnsProgramIds();
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const playerName = useAtomValue(PLAYER_NAME_ATOM);
  const [modalContract, setModalContract] = useState<ModalContract>(null);
  const [modalStrategy, setModalStrategy] = useState<ModalStrategy>(null);
  const [modalGameStatus, setModalGameStatus] = useState<ModalGameStatus>(null);
  const admin = useRef<null | HexString>(null);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ADMIN_ATOM);
  const { state, isStateRead } = useReadGameSessionState();
  const { sendMessage, sendPlayMessage } = useSyndoteMessage();
  const { checkBalance } = useCheckBalance();
  const strategyId = useAtomValue(CURRENT_STRATEGY_ID_ATOM);
  const [steps, setSteps] = useState<Step[]>([]);
  const [step, setStep] = useState(0);
  const { adminId, winner, gameStatus, entryFee, prizePool } = state || {};
  const isAdmin = account?.decodedAddress === adminId;
  const isGameStarted = steps.length > 0;
  const roll = steps[step];
  const strategyNeedsGas = gameStatus?.WaitingForGasForStrategy;
  const { properties, ownership } = roll || {};
  const playersArray = state?.players || [];

  const getPlayers = () => (isGameStarted && roll ? roll?.players : state?.players || []);

  const findPlayer = (address: string) => getPlayers().find(([newAddress]) => newAddress === address)?.[1];

  const players = playersArray.map(([address], index) => ({
    ...INIT_PLAYERS[index],
    address,
    ...(findPlayer(address) as PlayerState),
  }));
  const playersByStrategyAddress = players.reduce((acc, item) => {
    return {
      ...acc,
      [item.address]: item,
    };
  }, {}) as PlayersByStrategyAddress;
  const isAnyPlayer = players.length > 0;
  const playerStrategyId = players.find((player) => player.ownerId === account?.decodedAddress)?.address;

  const register = () => {
    const payload = { Register: { adminId, strategyId, name: playerName } };

    const onInBlock = () => {
      setCurrentGame('');
      setIsLoading(false);
    };
    const onError = () => setIsLoading(false);

    setIsLoading(true);
    sendMessage({
      payload,
      value: entryFee ? Number(withoutCommas(entryFee || '')) : undefined,
      onInBlock,
      onError,
    });
  };

  const startGame = () => {
    const payload = {
      Play: {
        adminId,
      },
    };

    const onInBlock = () => setIsLoading(false);
    const onError = () => setIsLoading(false);

    setIsLoading(true);

    checkBalance(
      730000000000,
      () => {
        sendPlayMessage({
          payload,
          gasLimit: 730000000000,
          onInBlock,
          onError,
        });
      },
      onError,
    );
  };

  const exitGame = () => {
    const payload = {
      ExitGame: {
        adminId,
      },
    };

    sendMessage({
      payload,
      onInBlock: () => {
        admin.current = null;
      },
    });
  };

  const addGasToPlayerStrategy = () => {
    const payload = {
      AddGasToPlayerStrategy: {
        adminId,
      },
    };

    sendMessage({
      payload,
      onInBlock: () => {
        setModalStrategy(null);
      },
    });
  };

  const continueGame = () => {
    setModalContract(null);
    startGame();
  };

  useEffect(() => {
    if (adminId) {
      admin.current = adminId;
    }
  }, [adminId]);

  useEffect(() => {
    if (gameStatus !== 'WaitingForGasForGameContract') {
      setModalContract(null);
      return;
    }

    if (isAdmin) {
      setModalContract('contractRequresGas');
      return;
    }

    setModalContract('adminReservesGas');
  }, [gameStatus, isAdmin]);

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
    if (gameStatus === 'Finished') {
      setModalGameStatus('gameFinished');
    }
  }, [gameStatus]);

  const getDecodedPayload = (payload: Bytes) => {
    if (!metadata) return;

    try {
      if (metadata?.types.others.output) {
        return metadata.createType(metadata?.types.others.output, payload).toHuman();
      }
    } catch (error) {
      console.log(error);
    }
  };

  const getDecodedPayloadHandle = (payload: Bytes) => {
    if (!metadata) return;

    try {
      if (metadata?.types.handle.output) {
        return metadata.createType(metadata?.types.handle.output, payload).toHuman();
      }
    } catch (error) {
      console.log(error);
    }
  };

  useEffect(() => {
    if (steps.length > 0) {
      setStep(steps.length - 1);
    }
  }, [steps]);

  const prevStep = () => setStep((prevValue) => (prevValue - 1 >= 0 ? prevValue - 1 : prevValue));
  const nextStep = () => setStep((prevValue) => (prevValue + 1 < steps.length ? prevValue + 1 : prevValue));
  const firstStep = () => setStep(0);
  const lastStep = () => setStep(steps.length - 1);

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (metadata) {
      unsub = api?.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }) => {
        const { message } = data;
        const { source, payload, destination } = message;

        if (source.toHex() === programId && destination.toHex() === admin.current) {
          const decodedPayload = getDecodedPayload(payload) as MessagePayload;

          if (typeof decodedPayload === 'object' && decodedPayload !== null) {
            if (decodedPayload.Step) {
              setSteps((prevSteps) => [...prevSteps, decodedPayload.Step]);
            }
          }

          const decodedPayloadHandle = getDecodedPayloadHandle(payload) as MessageHandlePayload;

          if (decodedPayloadHandle?.Ok && ['GameDeleted', 'GameWasCancelled'].includes(decodedPayloadHandle.Ok)) {
            if (admin.current !== account?.decodedAddress) {
              setModalGameStatus('gameCancelled');
            }

            admin.current = null;
            setSteps([]);
          }
        }
      });
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [metadata]);

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
      [...prevSteps].sort(({ currentStep }, { currentStep: anotherStep }) => +currentStep - +anotherStep),
    );
  }, [winner]);

  const handleCloseModal = () => {
    setModalGameStatus(null);
    setModalContract(null);
    setModalStrategy(null);
  };

  return isStateRead ? (
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
                  gameStatus === 'Registration' ? styles.controllerWhite : styles.controllerWithInnerBorder,
                )}>
                {isGameStarted && roll ? (
                  <Roll
                    color={getColor(roll.currentPlayer)}
                    player={roll.currentPlayer}
                    currentTurn={step + 1}
                    turnsAmount={steps.length}
                    onPrevClick={prevStep}
                    onNextClick={nextStep}
                    onFirstClick={firstStep}
                    onLastClick={lastStep}
                  />
                ) : (
                  <div className={clsx(styles.syndoteContainer, isAdmin && styles.syndoteContainerAdmin)}>
                    {gameStatus === 'Registration' && (
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
                            {playersArray.map((item) => item[1].ownerId).includes(account?.decodedAddress || '0x') ? (
                              <Button text="Cancel" color="grey" onClick={exitGame} />
                            ) : (
                              <Button text="Register" onClick={register} isLoading={isLoading} />
                            )}
                          </>
                        )}
                        {isAdmin && (
                          <>
                            <SessionInfo entryFee={state.entryFee} players={state.players} adminId={state.adminId} />
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
                    {gameStatus === 'Play' && (
                      <>
                        <div className={clsx(styles.headingWrapper, styles.headingWrapperAdmin)}>
                          <h1 className={styles.heading}>Registration...</h1>
                        </div>
                        {isAdmin && (
                          <>
                            <SessionInfo entryFee={state.entryFee} players={state.players} adminId={state.adminId} />
                            <div className={styles.mainButtons}>
                              <Button text="Start the game" onClick={startGame} className={styles.startGameButton} />
                            </div>
                          </>
                        )}
                        {!isAdmin && <span className={styles.subheading}>Waiting for admin to start the game...</span>}
                      </>
                    )}
                    {gameStatus === 'Finished' && (
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
              prizePool={prizePool}
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
                href="https://wiki.gear-tech.io/docs/examples/Gaming/monopoly/#%EF%B8%8F-build-master-and-player-programs"
                rel="noreferrer">
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
  ) : (
    <Loader />
  );
}

export { Home };

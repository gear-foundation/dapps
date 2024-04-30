import { useEffect, useRef, useState } from 'react';
import { useAtomValue, useSetAtom, useAtom } from 'jotai';
import { useAccount, useApi, withoutCommas } from '@gear-js/react-hooks';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { HexString } from '@polkadot/util/types';
import { ADDRESS, fields, INIT_PLAYERS } from 'consts';
import { MessageHandlePayload, MessagePayload, PlayerState, PlayersByStrategyAddress, Step } from 'types';
import meta from 'assets/meta/syndote_meta.txt';
import { UnsubscribePromise } from '@polkadot/api/types';
import { Loader } from 'components';
import { Bytes } from '@polkadot/types';
import { useProgramMetadata, useReadGameSessionState, useSyndoteMessage } from 'hooks/metadata';
import { Roll } from './roll';
import styles from './Home.module.scss';
import { Players } from './players/Players';
import { Button } from '@gear-js/vara-ui';
import { Cell } from './cell';
import { RequestGame } from 'pages/welcome/components/request-game';
import { CURRENT_GAME_ADMIN_ATOM, CURRENT_STRATEGY_ID_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from 'atoms';
import { SessionInfo } from './session-info';
import clsx from 'clsx';
import { TextModal } from './game-not-found-modal';
import { ContinueGameModal } from './continue-game-modal';
import { ReserveModal } from './reserve-modal';

function Home() {
  const { account } = useAccount();
  const { api } = useApi();
  const metadata = useProgramMetadata(meta);
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const playerName = useAtomValue(PLAYER_NAME_ATOM);
  const [isContinueGameModalOpen, setIsContinueGameModalOpen] = useState(false);
  const [isPlayerRemovedModalOpen, setIsPlayerRemovedModalOpen] = useState(false);
  const [isGameCancelledModalOpen, setIsGameCancelledModalOpen] = useState(false);
  const [isReserveModalOpen, setIsReserveModalOpen] = useState(false);
  const [isReserveInfoModalOpen, setIsReserveInfoModalOpen] = useState(false);
  const [isContinueGameInfoModalOpen, setIsContinueGameInfoModalOpen] = useState(false);
  const admin = useRef<null | HexString>(null);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ADMIN_ATOM);
  const { state, isStateRead } = useReadGameSessionState();
  const { isMeta, sendMessage, sendPlayMessage } = useSyndoteMessage();
  const { checkBalance } = useCheckBalance();
  const strategyId = useAtomValue(CURRENT_STRATEGY_ID_ATOM);
  const [steps, setSteps] = useState<Step[]>([]);
  const [step, setStep] = useState(0);
  const { adminId, winner, gameStatus, entryFee } = state || {};
  const isAdmin = account?.decodedAddress === adminId;
  const isGameStarted = steps.length > 0;
  const roll = steps[step];
  const strategyNeedsGas = gameStatus?.WaitingForGasForStrategy;
  const { properties, ownership } = roll || {};

  const playersArray = state?.players || [];

  const getPlayers = () => state?.players || [];

  const findPlayer = (address: string) => {
    console.log(getPlayers().find(([newAddress]) => newAddress === address));
    return getPlayers().find(([newAddress]) => newAddress === address)?.[1];
  };
  console.log('==STATE==');
  console.log(state);

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
  console.log(players);
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
  console.log('isRes');
  console.log(isReserveModalOpen);
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
        setIsReserveModalOpen(false);
      },
    });
  };

  const continueGame = () => {
    setIsContinueGameModalOpen(false);
    startGame();
  };

  useEffect(() => {
    console.log('SET ADMIN');
    console.log(adminId);
    if (adminId) {
      admin.current = adminId;
    }
  }, [adminId]);

  useEffect(() => {
    if (gameStatus !== 'WaitingForGasForGameContract') {
      setIsContinueGameInfoModalOpen(false);
      setIsContinueGameModalOpen(false);
      return;
    }

    if (isAdmin) {
      setIsContinueGameModalOpen(true);
      return;
    }

    setIsContinueGameInfoModalOpen(false);
  }, [gameStatus, isAdmin]);

  useEffect(() => {
    if (!strategyNeedsGas) {
      setIsReserveModalOpen(false);
      setIsReserveInfoModalOpen(false);
      return;
    }

    if (strategyNeedsGas === playerStrategyId) {
      setIsReserveModalOpen(true);
      return;
    }

    setIsReserveInfoModalOpen(true);
  }, [strategyNeedsGas, playerStrategyId]);
  console.log(api?.blockGasLimit.toNumber());
  console.log('ADMIN');
  console.log(admin);
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

        console.log(message.toHuman());

        console.log(source.toHex());
        console.log(ADDRESS.CONTRACT);
        console.log(destination.toHex());
        console.log(admin);

        if (source.toHex() === ADDRESS.CONTRACT && destination.toHex() === admin.current) {
          console.log('----------MESSAGE RECEIVED-----------');
          const decodedPayload = getDecodedPayload(payload) as MessagePayload;

          console.log('============ DECODED PAYLOAD ==============');
          console.log(decodedPayload);

          if (typeof decodedPayload === 'object' && decodedPayload !== null) {
            if (decodedPayload.Step) {
              setSteps((prevSteps) => [...prevSteps, decodedPayload.Step]);
            }
          }

          const decodedPayloadHandle = getDecodedPayloadHandle(payload) as MessageHandlePayload;

          console.log('============ DECODED HANDLE PAYLOAD ==============');
          console.log(decodedPayloadHandle);

          if (decodedPayloadHandle?.Ok && ['GameDeleted', 'GameWasCancelled'].includes(decodedPayloadHandle.Ok)) {
            console.log('-----------');
            console.log(admin.current);
            console.log(account?.decodedAddress);
            console.log(admin.current !== account?.decodedAddress);
            console.log('-----------');
            if (admin.current !== account?.decodedAddress) {
              console.log('lalalalalala');
              setIsGameCancelledModalOpen(true);
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
    setIsPlayerRemovedModalOpen(false);
    setIsGameCancelledModalOpen(false);
    setIsContinueGameModalOpen(false);
    setIsReserveModalOpen(false);
    setIsReserveInfoModalOpen(false);
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
          {isContinueGameModalOpen && <ContinueGameModal onReserve={continueGame} onClose={handleCloseModal} />}
          {isContinueGameInfoModalOpen && (
            <TextModal
              heading={`Game administrator reserves extra gas to continue the game`}
              text="Once he reserves the required amount, the game will continue."
              onClose={handleCloseModal}
            />
          )}
          {isReserveModalOpen && <ReserveModal onReserve={addGasToPlayerStrategy} onClose={handleCloseModal} />}
          {isReserveInfoModalOpen && (
            <TextModal
              heading={`Player ${playersByStrategyAddress[strategyNeedsGas || '0x']?.name} pays extra gas`}
              text="Once he reserves the required amount, the game will continue. If he fails to do so before the timer expires, he will lose and can only observe the game"
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
      {isGameCancelledModalOpen && (
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

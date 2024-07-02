import { memo, useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAtom, useAtomValue } from 'jotai';
import isEqual from 'lodash.isequal';
import { useAccount, useAlert, useApi, useHandleCalculateGas } from '@gear-js/react-hooks';
import { UnsubscribePromise } from '@polkadot/api/types';
import { UserMessageSent, decodeAddress } from '@gear-js/api';
import { Container, Footer } from '@dapps-frontend/ui';
import { useCheckBalance, useDnsProgramIds } from '@dapps-frontend/hooks';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import styles from './Layout.module.scss';
import { cx, getDecodedReply, logger, withoutCommas } from '@/utils';
import { Heading } from '../Heading';
import { Road } from '../Road';
import { Button } from '@/ui';
import accelerateSVG from '@/assets/icons/accelerate-icon.svg';
import shootSVG from '@/assets/icons/shoot-icon.svg';
import { ReactComponent as GearLogoIcon } from '@/assets/icons/gear-logo-icon.svg';
import { CURRENT_GAME, IS_CURRENT_GAME_READ_ATOM, IS_SUBSCRIBED_ATOM } from '@/atoms';
import { usePlayerMoveMessage, useStartGameMessage } from '../../hooks';
import { Loader } from '@/components';
import { MessageDetails, RepliesQueue, UserMessage, WinStatus } from './Layout.interface';
import { PLAY } from '@/App.routes';
import { ContractError, DecodedReplyItem, GameState } from '@/types';
import { ADDRESS } from '@/consts';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import {
  CURRENT_SENT_MESSAGE_ID_ATOM,
  IS_STARTING_NEW_GAME_ATOM,
  IS_STATE_READ_ATOM,
  REPLY_DATA_ATOM,
} from '../../atoms';

function LayoutComponent() {
  const { signless, gasless } = useEzTransactions();
  const [currentGame, setCurrentGame] = useAtom(CURRENT_GAME);
  const isCurrentGameRead = useAtomValue(IS_CURRENT_GAME_READ_ATOM);
  const [isPlayerAction, setIsPlayerAction] = useState<boolean>(true);
  const [isLoading, setIsLoading] = useAtom(IS_STARTING_NEW_GAME_ATOM);
  const [isRoadLoaded, setIsRoadLoaded] = useState(false);
  const { isAvailableBalanceReady } = useAccountAvailableBalance();
  const { account } = useAccount();
  const alert = useAlert();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const navigate = useNavigate();
  const sendPlayerMoveMessage = usePlayerMoveMessage();
  const { meta, message: startGameMessage } = useStartGameMessage();
  const { programId } = useDnsProgramIds();
  const calculateGas = useHandleCalculateGas(programId, meta);
  const [isStateRead, setIsStateRead] = useAtom(IS_STATE_READ_ATOM);
  const { api } = useApi();

  const messageSubscription: React.MutableRefObject<UnsubscribePromise | null> = useRef(null);
  const repliesQueue: React.MutableRefObject<RepliesQueue> = useRef([]);
  const [replyData, setReplyData] = useAtom(REPLY_DATA_ATOM);
  const [currentSentMessageId, setCurrentSentMessageId] = useAtom(CURRENT_SENT_MESSAGE_ID_ATOM);
  const [isSubscribed, setIsSubscribed] = useAtom(IS_SUBSCRIBED_ATOM);

  const handleUnsubscribeFromEvent = (onSuccess?: () => void) => {
    if (messageSubscription.current) {
      messageSubscription.current?.then((unsubCallback) => {
        unsubCallback();
        logger('UNsubscribed from reply');
        setIsSubscribed(false);
        onSuccess?.();
      });
    }
  };

  const decodePair = useCallback(
    (i: number) => {
      logger('triggers SentMessageId Effect');

      if (i > 2) {
        setIsStateRead(false);
        setIsLoading(false);
      }

      if (currentSentMessageId) {
        logger(`SentMessageId exists: ${currentSentMessageId}`);
        logger(repliesQueue.current);
        const foundRepliesPair = repliesQueue.current.find(
          (item) => (item.auto?.toHuman().details as MessageDetails).to === currentSentMessageId,
        );

        logger(`Reply Pair found:`);
        logger({ auto: foundRepliesPair?.auto?.toHuman(), manual: foundRepliesPair?.manual?.toHuman() });
        logger(`Reply found: ${foundRepliesPair?.manual}`);

        if (foundRepliesPair?.auto?.toHuman() && foundRepliesPair.manual?.toHuman()) {
          const { manual } = foundRepliesPair;

          logger('trying to decode....:');
          try {
            const reply = getDecodedReply(manual.payload, meta);
            logger('DECODED message successfully');
            logger('new reply HAS COME:');
            logger(reply);

            if (reply && reply.cars.length && !isEqual(reply?.cars, replyData?.cars)) {
              logger('prev reply state:');
              logger(replyData);
              logger('new reply UPDATED and going to state:');
              logger(reply);
              setReplyData(reply);
              setCurrentSentMessageId(null);
              handleUnsubscribeFromEvent();
            }
          } catch (e) {
            logger(e);
            alert.error((e as ContractError).message);
          }
        }

        if (foundRepliesPair && foundRepliesPair.auto?.toHuman() && !foundRepliesPair.manual?.toHuman()) {
          setCurrentSentMessageId(null);
          setIsPlayerAction(true);
          handleUnsubscribeFromEvent();

          if (isLoading) {
            setIsStateRead(false);
            setIsLoading(false);
          }
        }

        if (!foundRepliesPair?.auto?.toHuman()) {
          console.log(`reply not found, retrying(${i + 1})`);
          setTimeout(() => decodePair(i + 1), 2000);
        }
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [currentSentMessageId],
  );

  useEffect(() => {
    decodePair(0);
  }, [decodePair]);

  const handleChangeState = ({ data: _data }: UserMessageSent) => {
    const { message } = _data;

    const { destination, source, details: messageDetails, id } = message as UserMessage;

    const signlessPairAddress = signless.pair && decodeAddress(signless.pair.address);
    const isOwner = destination.toHex() === account?.decodedAddress || destination.toHex() === signlessPairAddress;
    const isCurrentProgram = source.toHex() === programId;

    const details = messageDetails.toHuman() as MessageDetails;

    if (isOwner && isCurrentProgram) {
      if (details?.to && !repliesQueue.current.map((item) => item.auto?.toHuman().id).includes(id.toHex())) {
        console.log('pushed');
        console.log(repliesQueue.current.map((item) => (item.auto?.toHuman()?.details as MessageDetails).to));

        repliesQueue.current.push({ auto: message, manual: null });
      }

      if (!details && !repliesQueue.current[repliesQueue.current.length - 1].manual) {
        console.log('pushed2');

        repliesQueue.current[repliesQueue.current.length - 1].manual = message;
      }
      logger(repliesQueue.current.map((item) => ({ auto: item.auto?.toHuman(), manual: item.manual?.toHuman() })));
    }
  };
  // eslint-disable-next-line react-hooks/exhaustive-deps

  const handleSubscribeToEvent = useCallback(async () => {
    if (api && meta && !isSubscribed) {
      messageSubscription.current = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleChangeState);
      setIsSubscribed(true);
      logger('Subscribed on reply');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, isSubscribed, meta, signless.pair?.address]);

  const defineStrategyAction = (type: 'accelerate' | 'shoot') => {
    if (type === 'accelerate') {
      return 'BuyAcceleration';
    }

    if (type === 'shoot') {
      return 'BuyShell';
    }
  };

  const handleActionChoose = async (type: 'accelerate' | 'shoot') => {
    setIsPlayerAction(false);
    logger(`CLICK ACTION ${type}`);
    logger(`Disabling actions`);
    const payload = {
      PlayerMove: {
        strategy_action: defineStrategyAction(type),
      },
    };

    handleSubscribeToEvent();

    let { voucherId } = gasless;
    if (account && gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    const getOnError = (error: string) => () => {
      setIsPlayerAction(true);
      handleUnsubscribeFromEvent();
      logger(error);
    };

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);
        logger(`Calculating gas:`);
        logger(`MIN_LIMIT ${min_limit}`);
        logger(`LIMIT ${gasLimit}`);
        logger(`Calculated gas SUCCESS`);
        logger(`Sending message`);
        console.log(`START TURN ${Number(currentGame?.currentRound) + 1}`);

        const sendMessage = () =>
          sendPlayerMoveMessage({
            payload,
            gasLimit,
            voucherId,
            onError: getOnError(`Errror send message`),
            onSuccess: (messageId) => {
              logger(`sucess on ID: ${messageId}`);
            },
            onInBlock: (messageId) => {
              logger('messageInBlock');
              logger(`messageID: ${messageId}`);
              setCurrentSentMessageId(messageId);
            },
          });

        if (voucherId) {
          sendMessage();
        } else {
          checkBalance(gasLimit, sendMessage, getOnError(`Errror check balance`));
        }
      })
      .catch((error) => {
        logger(error);
        setIsPlayerAction(true);
        handleUnsubscribeFromEvent();
        alert.error('Gas calculation error');
      });
  };

  const defineWinStatus = (): WinStatus => {
    if (currentGame?.state === 'Finished') {
      return currentGame.result;
    }

    return null;
  };

  const handleStartNewGame = useCallback(
    async (startManually?: boolean) => {
      if (meta && isCurrentGameRead && !isLoading && (!currentGame || startManually)) {
        const payload = {
          StartGame: {},
        };

        handleSubscribeToEvent();

        const onError = (error?: unknown) => {
          handleUnsubscribeFromEvent();
          setIsStateRead(true);
          setIsLoading(false);
          logger(error || 'error');
          navigate(PLAY, { replace: true });
        };

        setIsPlayerAction(false);
        setIsLoading(true);
        setIsStateRead(false);

        let { voucherId } = gasless;
        if (account && gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
          voucherId = await gasless.requestVoucher(account.address);
        }

        calculateGas(payload)
          .then((res) => res.toHuman())
          .then(({ min_limit }) => {
            const minLimit = withoutCommas(min_limit as string);
            const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

            const sendMessage = () => {
              startGameMessage({
                payload,
                gasLimit,
                voucherId,
                onInBlock: (messageId) => {
                  logger('Start Game messageInBlock');
                  logger(`messageID: ${messageId}`);
                  setCurrentSentMessageId(messageId);
                },
                onError,
              });
            };

            if (voucherId) {
              sendMessage();
            } else {
              checkBalance(gasLimit, sendMessage, onError);
            }
          })
          .catch((error) => {
            alert.error('Gas calculation error');
            onError(error);
          });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [meta, currentGame, isCurrentGameRead, account, gasless, signless, handleSubscribeToEvent],
  );

  useEffect(() => {
    if (isStateRead) {
      setIsPlayerAction(true);
    }
  }, [isStateRead]);

  useEffect(() => {
    handleStartNewGame();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [meta, isCurrentGameRead]);

  useEffect(() => {
    if (replyData && currentGame) {
      const { cars, result } = replyData;
      logger('Updates state to new reply');

      setCurrentGame(() =>
        cars.reduce((acc: GameState, item: DecodedReplyItem) => {
          const [address, position, effect] = item;

          return {
            ...acc,
            cars: {
              ...acc.cars,
              [address]: {
                ...acc.cars[address],
                position,
                roundResult: effect,
              },
            },
          };
        }, currentGame),
      );
      setCurrentGame((prev) =>
        prev
          ? {
              ...prev,
              result,
              state: result ? 'Finished' : prev.state,
              currentRound: String(Number(prev.currentRound) + 1),
            }
          : null,
      );
      logger('Enabling actions');
      setIsPlayerAction(true);
      logger(`END OF TURN ${Number(currentGame.currentRound) + 1}`);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [replyData]);

  const handleRoadLoaded = () => {
    setIsRoadLoaded(true);
  };

  return (
    <>
      {currentGame && account?.decodedAddress && isAvailableBalanceReady && !isLoading && isStateRead ? (
        <div className={cx(styles.container, currentGame.state !== 'Finished' ? styles['container-flexed'] : '')}>
          {isRoadLoaded && (
            <Heading
              currentTurn={currentGame.currentRound}
              isPlayerAction={isPlayerAction}
              winStatus={defineWinStatus()}
            />
          )}
          <Road newCars={currentGame.cars} carIds={currentGame.carIds} onRoadLoaded={handleRoadLoaded} />
          {isRoadLoaded && (
            <>
              {currentGame.state !== 'Finished' && (
                <div className={cx(styles.controls)}>
                  <Button
                    label="Accelerate"
                    variant="primary"
                    size="large"
                    icon={accelerateSVG}
                    disabled={!isPlayerAction}
                    isLoading={!account.decodedAddress || !meta}
                    className={cx(styles['control-button'])}
                    onClick={() => handleActionChoose('accelerate')}
                  />
                  <Button
                    label="Shoot"
                    variant="primary"
                    size="large"
                    icon={shootSVG}
                    disabled={!isPlayerAction}
                    isLoading={!account.decodedAddress || !meta}
                    className={cx(styles['control-button'], styles['control-button-red'])}
                    onClick={() => handleActionChoose('shoot')}
                  />
                </div>
              )}
              {currentGame.state === 'Finished' && (
                <div className={cx(styles['rewards-wrapper'])}>
                  <Button
                    variant="primary"
                    label="Play again"
                    size="large"
                    isLoading={gasless.isLoading}
                    className={cx(styles.btn)}
                    onClick={() => handleStartNewGame(true)}
                  />
                </div>
              )}
              {currentGame.state !== 'Finished' && (
                <Container className={cx(styles.footer)}>
                  <Footer vara />
                </Container>
              )}
              {currentGame.state === 'Finished' && (
                <div className={cx(styles['footer-wrapper'])}>
                  <div className={styles.banner}>
                    <div className={styles.banner__right}>
                      <h2 className={styles.banner__title}>
                        Thank you for your interest <span>in the Vara Network.</span>
                      </h2>
                      <div className={styles.banner__text}>
                        <p>You&apos;ve experienced a fully on-chain game.</p>
                        <p>
                          We look forward to having you join the ranks of developers shaping the new generation of
                          decentralized applications.
                        </p>
                      </div>
                    </div>
                    <ul className={styles.banner__left}>
                      <li className={styles.banner__item}>
                        <div className={styles.banner__icon}>
                          <GearLogoIcon width={24} height={24} />
                        </div>
                        <p className={styles['visit-block']}>
                          Visit the{' '}
                          <a href="https://wiki.gear-tech.io/" target="_blank" rel="noreferrer">
                            Gear Wiki
                          </a>{' '}
                          to discover how easy it is to create programs using the Gear Protocol.
                        </p>
                      </li>
                      <li className={styles.banner__item}>
                        <div className={styles.banner__icon}>
                          <GearLogoIcon width={24} height={24} />
                        </div>
                        <p className={styles['visit-block']}>
                          Consider enrolling in a free course at{' '}
                          <a href="https://academy.gear.foundation/" target="_blank" rel="noreferrer">
                            Gear&nbsp;Academy
                          </a>{' '}
                          to become a top-notch Web3 developer.
                        </p>
                      </li>
                    </ul>
                  </div>
                  <Container>
                    <Footer vara />
                  </Container>
                </div>
              )}
            </>
          )}
        </div>
      ) : (
        <div className={styles['loader-wrapper']}>
          <Loader />
        </div>
      )}
    </>
  );
}

const Layout = memo(LayoutComponent, isEqual);

export { Layout };

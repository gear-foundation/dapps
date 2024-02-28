import { useEffect, useState } from 'react';
import { useAccount, useApi, useReadFullState, useSendMessageHandler } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { ADDRESS, fields, INIT_PLAYERS, LocalStorage } from 'consts';
import { MessagePayload, State, Step } from 'types';
import meta from 'assets/meta/syndote_meta.txt';
import { UnsubscribePromise } from '@polkadot/api/types';
import { Loader } from 'components';
import { Bytes } from '@polkadot/types';
import { useProgramMetadata, useReadGameSessionState, useSyndoteMessage } from 'hooks/metadata';
import { Roll } from './roll';
import styles from './Home.module.scss';
import { Players } from './players/Players';
import { Button } from '@gear-js/vara-ui';
import { Buttons } from './buttons';
import { Cell } from './cell';
import { RequestGame } from 'pages/welcome/components/request-game';

function Home() {
  const { account, logout } = useAccount();

  const [programId, setProgramId] = useState((localStorage[LocalStorage.Player] ?? '') as HexString);
  const resetProgramId = () => setProgramId('' as HexString);

  useEffect(() => {
    localStorage.setItem(LocalStorage.Player, programId);
  }, [programId]);

  const { api } = useApi();

  const metadata = useProgramMetadata(meta);
  const { state, isStateRead } = useReadGameSessionState();

  const { isMeta, sendMessage } = useSyndoteMessage();

  /////////////////////////////////////////////////////////////////////////////////////////////////

  const register = (player: HexString) =>
    sendMessage({ payload: { Register: { player } }, onSuccess: () => setProgramId(player) });

  /////////////////////////////////////////////////////////////////////////////////////////////////

  const startGame = () => sendMessage({ payload: { Play: null } });

  const { adminId } = state || {};
  const isAdmin = account?.decodedAddress === adminId;

  const getDecodedPayload = (payload: Bytes) => {
    if (!metadata) return;

    const { output } = metadata.types.handle;

    if (output === null || output === undefined) return;

    // handle.output is specific for contract
    return metadata.createType(output, payload).toHuman();
  };

  const [steps, setSteps] = useState<Step[]>([]);
  const isGameStarted = steps.length > 0;

  const [step, setStep] = useState(0);

  useEffect(() => {
    if (steps.length > 0) setStep(steps.length - 1);
  }, [steps]);

  const prevStep = () => setStep((prevValue) => (prevValue - 1 >= 0 ? prevValue - 1 : prevValue));
  const nextStep = () => setStep((prevValue) => (prevValue + 1 < steps.length ? prevValue + 1 : prevValue));
  const firstStep = () => setStep(0);
  const lastStep = () => setStep(steps.length - 1);

  const roll = steps[step];
  const { properties, ownership } = roll || {};

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (metadata) {
      unsub = api?.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }) => {
        const { message } = data;
        const { source, payload } = message;

        if (source.toHex() === ADDRESS.CONTRACT) {
          const decodedPayload = getDecodedPayload(payload) as MessagePayload;

          if (typeof decodedPayload === 'object' && decodedPayload !== null) {
            // @ts-ignore
            if (decodedPayload.Step) {
              // @ts-ignore
              setSteps((prevSteps) => [...prevSteps, decodedPayload.Step]);
            }
          }
        }
      });
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [metadata]);

  const getPlayers = () => (isGameStarted ? roll.players : state!.players!);

  const playersArray = state?.players || [];

  const players = playersArray.map(([address], index) => ({
    ...INIT_PLAYERS[index],
    address,
    ...getPlayers().find(([newAddress]) => newAddress === address)![1],
  }));

  const isAnyPlayer = players.length > 0;

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

  const { winner } = state || {};

  useEffect(() => {
    if (!winner || winner.startsWith('0x00')) return;

    setSteps((prevSteps) =>
      [...prevSteps].sort(({ currentStep }, { currentStep: anotherStep }) => +currentStep - +anotherStep),
    );
  }, [winner]);

  return isStateRead ? (
    <>
      {!!state && (
        <>
          <div className={styles.players}>
            {isAnyPlayer && <Players list={players} winner={winner} />}
            <Button text="Exit" onClick={resetProgramId} />
          </div>

          <div className={styles.field}>
            <div className={styles.wrapper}>
              {getFields()}
              <div className={styles.controller}>
                {isGameStarted ? (
                  <Roll
                    color={getColor(roll.currentPlayer)}
                    player={roll.currentPlayer}
                    currentTurn={step + 1}
                    turnsAmount={steps.length}
                    onPrevClick={prevStep}
                    onNextClick={nextStep}
                    onFirstClick={firstStep}
                    onLastClick={lastStep}
                    onMainClick={isAdmin ? startGame : undefined}
                  />
                ) : (
                  <>
                    <h1 className={styles.heading}>Syndote Game</h1>
                    <p className={styles.subheading}>
                      {isAdmin ? 'Press play to start' : 'Waiting for admin to start a game'}
                    </p>
                    <Buttons onMainClick={isAdmin ? startGame : undefined} />
                  </>
                )}
              </div>
            </div>
          </div>
        </>
      )}
      {!state && (
        <div className={styles.container}>
          <div className={styles.requestGameContainer}>
            <RequestGame
            // onSubmit={register} onBack={logout}
            />
          </div>
          <div className={styles.downloadGameContainer}>
            <h4 className={styles.donwloadTitle}>Get Started</h4>
            <p className={styles.donwloadText}>
              To quickly get started, download the default Wasm program of the game, upload it to the blockchain
              network, and then copy its address to specify it in the game
            </p>
            <div className={styles.donwloadButtons}>
              <Button color="transparent" text="Download file" />
              <Button color="transparent" text="How does it work?" />
            </div>
          </div>
        </div>
      )}
    </>
  ) : (
    <Loader />
  );
}

export { Home };

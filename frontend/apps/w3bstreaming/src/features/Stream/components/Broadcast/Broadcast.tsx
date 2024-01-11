import { MutableRefObject, useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import styles from './Broadcast.module.scss';
import { cx } from '@/utils';
import { RTC_CONFIG } from '../../config';
import { Player } from '../Player';
import { Button } from '@/ui';

import StreamSignalSVG from '@/assets/icons/signal-stream-icon.svg';
import { MediaStreamSequence } from '../../utils';
import { BroadcastProps, AnswerMsg, CandidateMsg, WatchMsg, StreamStatus, StreamType } from './Broadcast.interface';
import { ADDRESS } from '@/consts';

function Broadcast({ socket, streamId }: BroadcastProps) {
  const { account } = useAccount();
  const navigate = useNavigate();

  const localVideo: MutableRefObject<HTMLVideoElement | null> = useRef(null);
  const conns: MutableRefObject<Record<string, RTCPeerConnection>> = useRef({});
  const commonStream: MutableRefObject<MediaStream> = useRef(new MediaStream());
  const mediaTrackSequence: MutableRefObject<MediaStreamSequence> = useRef(new MediaStreamSequence()); //to store the order of media tracks in commonStream.current

  const micTransceiver: MutableRefObject<Record<string, RTCRtpTransceiver | null>> = useRef({});
  const camTransceiver: MutableRefObject<Record<string, RTCRtpTransceiver | null>> = useRef({});
  const scrCaptureTransceiver: MutableRefObject<Record<string, RTCRtpTransceiver | null>> = useRef({});
  const scrAudioTransceiver: MutableRefObject<Record<string, RTCRtpTransceiver | null>> = useRef({});

  const [localStream, setLocalStream] = useState<MediaStream | null>(null);
  const [isSoundMuted, setIsSoundMuted] = useState<boolean>(false);
  const [isCameraBlocked, setIsCameraBlocked] = useState<boolean>(false);
  const [streamStatus, setStreamStatus] = useState<StreamStatus>('loading');
  const [streamType, setStreamType] = useState<StreamType>('camera');

  const handleGetIsAlreadyHaveStream = async (address: string) => {
    const res = await fetch(`${ADDRESS.BACKEND_SERVER}/is-already-having-stream?address=${address}`);
    const isHave = await res.json();

    return isHave;
  };

  const handleCheckIsAlreadyHaveStream = useCallback(async () => {
    if (account?.decodedAddress) {
      const isHave = await handleGetIsAlreadyHaveStream(account?.decodedAddress);

      if (isHave) {
        setStreamStatus('already-have');
        return;
      }

      setStreamStatus('not-started');
    }
  }, [account?.decodedAddress]);

  useEffect(() => {
    handleCheckIsAlreadyHaveStream();
  }, [handleCheckIsAlreadyHaveStream]);

  const handleGoToAccountPage = () => {
    navigate('/account');
  };

  const updatesDevices = async () => {
    try {
      const devices = await navigator.mediaDevices.enumerateDevices();

      const requestedStream = await navigator.mediaDevices.getUserMedia({
        video: devices.some((device) => device.kind === 'videoinput'),
        audio: devices.some((device) => device.kind === 'audioinput'),
      });

      const sequence = mediaTrackSequence.current;

      //stops current audio and video
      const microphoneIndex = sequence.getIndex('microphone');

      if (microphoneIndex !== undefined && commonStream.current.getTracks()[microphoneIndex]) {
        commonStream.current.removeTrack(commonStream.current.getTracks()[microphoneIndex]);
        sequence.removeByType('microphone');
      }

      Object.keys(conns.current).forEach((id) => {
        micTransceiver.current[id]?.stop();
      });

      const cameraIndex = sequence.getIndex('camera');

      if (cameraIndex !== undefined && commonStream.current.getTracks()[cameraIndex]) {
        commonStream.current.removeTrack(commonStream.current.getTracks()[cameraIndex]);
        sequence.removeByType('camera');
      }

      Object.keys(conns.current).forEach((id) => {
        camTransceiver.current[id]?.stop();
      });

      //sets new ones
      const micTrack = requestedStream.getAudioTracks()?.[0];

      if (micTrack) {
        commonStream.current.addTrack(micTrack);
        sequence.add('microphone');

        Object.keys(conns.current).forEach((id) => {
          micTransceiver.current[id] = conns.current[id].addTransceiver(micTrack, {
            direction: 'sendonly',
            streams: [commonStream.current],
          });
        });
        setIsSoundMuted(false);
      } else {
        setIsSoundMuted(true);
      }

      const camTrack = requestedStream.getVideoTracks()?.[0];

      if (camTrack) {
        commonStream.current.addTrack(camTrack);
        sequence.add('camera');

        Object.keys(conns.current).forEach((id) => {
          camTransceiver.current[id] = conns.current[id].addTransceiver(camTrack, {
            direction: 'sendonly',
            streams: [commonStream.current],
          });
        });
        setIsCameraBlocked(false);
      } else {
        setIsCameraBlocked(true);
      }

      setLocalStream(requestedStream);
    } catch (error) {
      console.log(error);
      //if no devices at all then they're got removed from commonStream.current automatically
      //we need just remove them from sequence
      const sequence = mediaTrackSequence.current;

      sequence.removeByType('microphone');
      sequence.removeByType('camera');
      setIsSoundMuted(true);
      setIsCameraBlocked(true);
    }
  };

  useEffect(() => {
    navigator.mediaDevices.addEventListener('devicechange', () => {
      updatesDevices();
    });
  }, []);

  const loadDevices = async () => {
    //gets the list of available devices
    const devices = await navigator.mediaDevices.enumerateDevices();
    //finds some video and audio devices from devices and puts them together as a stream
    const requestedStream = await navigator.mediaDevices.getUserMedia({
      video: devices.some((device) => device.kind === 'videoinput'),
      audio: devices.some((device) => device.kind === 'audioinput'),
    });
    const sequence = mediaTrackSequence.current;

    const micTrack = requestedStream.getAudioTracks()?.[0];

    if (micTrack) {
      commonStream.current.addTrack(micTrack);
      sequence.add('microphone');
    } else {
      setIsSoundMuted(true);
    }

    const camTrack = requestedStream.getVideoTracks()?.[0];

    if (camTrack) {
      commonStream.current.addTrack(camTrack);
      sequence.add('camera');
    } else {
      setIsCameraBlocked(true);
    }

    setLocalStream(requestedStream);
  };

  const handleScreenShare = async () => {
    if (streamType === 'screen') {
      return;
    }
    if (streamType === 'camera') {
      try {
        setStreamType('screen');

        const screenStream = await navigator.mediaDevices.getDisplayMedia({ audio: true, video: true });

        if (!screenStream.getTracks().length) {
          setStreamType('camera');
          return;
        }

        const sequence = mediaTrackSequence.current;

        //replaces camera remote track to null
        const indexOfCameraTrack = sequence.getIndex('camera');
        if (indexOfCameraTrack !== undefined) {
          Object.keys(conns.current).forEach((id) => {
            if (camTransceiver.current[id]) {
              camTransceiver.current[id]?.stop();
            }
          });
          commonStream.current.getTracks()[indexOfCameraTrack].enabled = false;
        }

        //adds or replaces screenSound remote tracks to value
        const requestedScreenAudioTrack = screenStream.getAudioTracks()?.[0];
        const indexOfExistingScreenAudioTrack = sequence.getIndex('screenSound');

        if (indexOfExistingScreenAudioTrack === undefined && requestedScreenAudioTrack) {
          sequence.add('screenSound');
          commonStream.current.addTrack(requestedScreenAudioTrack);
          Object.keys(conns.current).forEach((id) => {
            scrAudioTransceiver.current[id] = conns.current[id]?.addTransceiver(requestedScreenAudioTrack, {
              direction: 'sendonly',
              streams: [commonStream.current],
            });
          });
        }

        //adds or replaces screenCapture remote tracks to value
        const requestedScreenCaptureTrack = screenStream.getVideoTracks()?.[0];
        const indexOfExistingScreenCaptureTrack = sequence.getIndex('screenCapture');

        if (indexOfExistingScreenCaptureTrack === undefined && requestedScreenCaptureTrack) {
          sequence.add('screenCapture');
          commonStream.current.addTrack(requestedScreenCaptureTrack);
          Object.keys(conns.current).forEach((id) => {
            scrCaptureTransceiver.current[id] = conns.current[id]?.addTransceiver(requestedScreenCaptureTrack, {
              direction: 'sendonly',
              streams: [commonStream.current],
            });
          });
        }

        //creates new local stream
        const indexes = sequence.getIndexes(['microphone', 'screenSound', 'screenCapture']);

        setLocalStream(
          () => new MediaStream(indexes.map((index) => commonStream.current.getTracks()[index as number])),
        );

        screenStream.getTracks()[0].onended = () => {
          //replacing screenSound and screenCapture remote tracks to null

          const audInd = sequence.getIndex('screenSound');
          if (audInd) {
            Object.keys(conns.current).forEach((id) => {
              scrAudioTransceiver.current[id]?.stop();
              scrAudioTransceiver.current[id] = null;
            });
            commonStream.current.removeTrack(commonStream.current.getTracks()[audInd]);
            sequence.removeByType('screenSound');
          }

          const capInd = sequence.getIndex('screenCapture');
          if (capInd) {
            Object.keys(conns.current).forEach((id) => {
              scrCaptureTransceiver.current[id]?.stop();
              scrCaptureTransceiver.current[id] = null;
            });
            commonStream.current.removeTrack(commonStream.current.getTracks()[capInd]);
            sequence.removeByType('screenCapture');
          }

          //replacing camera remote track to value
          if (indexOfCameraTrack) {
            commonStream.current.getTracks()[indexOfCameraTrack].enabled = true;
            Object.keys(conns.current).forEach((id) => {
              camTransceiver.current[id] = conns.current[id].addTransceiver(
                commonStream.current.getTracks()[indexOfCameraTrack].clone(),
                {
                  direction: 'sendonly',
                  streams: [commonStream.current],
                },
              );
            });
          }

          const newRequiredIndexes = sequence.getIndexes(['microphone', 'camera']);
          setLocalStream(
            () => new MediaStream(newRequiredIndexes.map((index) => commonStream.current.getTracks()[index as number])),
          );
          setStreamType('camera');
        };
      } catch (err) {
        setStreamType('camera');
        console.log(err);
      }
    }
  };

  const handleMuteSound = (isMuted: boolean) => {
    const sequence = mediaTrackSequence.current;
    const indexOfMicrophone = sequence.getIndex('microphone');

    if (isMuted) {
      if (indexOfMicrophone !== undefined) {
        Object.keys(conns.current).forEach((id) => {
          const transceiver = micTransceiver.current[id];

          if (transceiver?.sender.track) {
            transceiver.sender.track.enabled = true;
          }
        });
        commonStream.current.getTracks()[indexOfMicrophone].enabled = true;
        setIsSoundMuted(() => false);
      }
    }
    if (!isMuted) {
      if (indexOfMicrophone !== undefined) {
        Object.keys(conns.current).forEach((id) => {
          const transceiver = micTransceiver.current[id];

          if (transceiver?.sender.track) {
            transceiver.sender.track.enabled = false;
          }
        });
        commonStream.current.getTracks()[indexOfMicrophone].enabled = false;
        setIsSoundMuted(() => true);
      }
    }
  };

  const handleBlockCamera = (isBlocked: boolean) => {
    if (streamType === 'camera') {
      const sequence = mediaTrackSequence.current;
      const indexOfCamera = sequence.getIndex('camera');

      if (isBlocked) {
        if (indexOfCamera !== undefined) {
          Object.keys(conns.current).forEach((id) => {
            const transceiver = camTransceiver.current[id];

            if (transceiver?.sender.track) {
              transceiver.sender.track.enabled = true;
            }
          });
          commonStream.current.getTracks()[indexOfCamera].enabled = true;
        }
      }

      if (!isBlocked) {
        if (indexOfCamera !== undefined) {
          Object.keys(conns.current).forEach((id) => {
            const transceiver = camTransceiver.current[id];

            if (transceiver?.sender.track) {
              transceiver.sender.track.enabled = false;
            }
          });
          commonStream.current.getTracks()[indexOfCamera].enabled = false;
        }
      }

      setIsCameraBlocked((prev) => !prev);
    }
  };

  const startStream = async () => {
    if (!account?.decodedAddress) {
      return;
    }

    setStreamStatus('loading');

    const isHavingAnotherStream = await handleGetIsAlreadyHaveStream(account?.decodedAddress);

    if (isHavingAnotherStream) {
      setStreamStatus('already-have');
      return;
    }

    try {
      loadDevices();

      socket.emit('broadcast', account?.decodedAddress, { streamId });

      socket.on('watch', (idOfWatcher: string, msg: WatchMsg) => {
        conns.current[idOfWatcher] = new RTCPeerConnection(RTC_CONFIG);
        const sequence = mediaTrackSequence.current;

        const micIndex = sequence.getIndex('microphone');

        if (micIndex !== undefined) {
          micTransceiver.current[idOfWatcher] = conns.current[idOfWatcher]?.addTransceiver(
            commonStream.current.getTracks()[micIndex].clone(),
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        const camIndex = sequence.getIndex('camera');

        if (camIndex !== undefined) {
          if (commonStream.current.getTracks()[camIndex].enabled) {
            camTransceiver.current[idOfWatcher] = conns.current[idOfWatcher]?.addTransceiver(
              commonStream.current.getTracks()[camIndex].clone(),
              {
                direction: 'sendonly',
                streams: [commonStream.current],
              },
            );
          }
        }

        const scrSoundIndex = sequence.getIndex('screenSound');

        if (scrSoundIndex !== undefined) {
          scrAudioTransceiver.current[idOfWatcher] = conns.current[idOfWatcher].addTransceiver(
            commonStream.current.getTracks()[scrSoundIndex].clone(),
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        const scrCaptureIndex = sequence.getIndex('screenCapture');

        if (scrCaptureIndex !== undefined) {
          scrAudioTransceiver.current[idOfWatcher] = conns.current[idOfWatcher].addTransceiver(
            commonStream.current.getTracks()[scrCaptureIndex].clone(),
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        conns.current[idOfWatcher]!.onicecandidate = (event: RTCPeerConnectionIceEvent) => {
          if (event.candidate) {
            socket.emit('candidate', idOfWatcher, { id: account.address, candidate: event.candidate });
          }
        };

        conns.current[idOfWatcher]!.onnegotiationneeded = () => {
          conns.current[idOfWatcher]!.createOffer()
            .then((offer) => conns.current[idOfWatcher]?.setLocalDescription(offer))
            .then(() =>
              socket.emit('offer', account?.decodedAddress, {
                description: conns.current[idOfWatcher]?.localDescription,
                userId: idOfWatcher,
                streamId: msg.streamId,
              }),
            )
            .catch(() => {
              console.log('error when setLocalDescription');
            });
        };
      });

      socket.on('candidate', (userId: string, { candidate }: CandidateMsg) => {
        conns.current[userId]?.addIceCandidate(new RTCIceCandidate(candidate)).catch((e: any) => console.error(e));
      });

      socket.on('answer', (_: string, { userId, description }: AnswerMsg) => {
        conns.current[userId]?.setRemoteDescription(description).catch(() => {
          console.log('error when setRemoteDescription');
        });
      });
    } catch (error) {
      if (
        (error as Error).message ===
        `Failed to execute 'getUserMedia' on 'MediaDevices': At least one of audio and video must be requested`
      ) {
        alert('At least one of audio and video must be');
      }
    }
  };

  const handleStopStream = () => {
    if (localStream) {
      localStream?.getTracks().forEach((track) => track.stop());
    }
    Object.keys(conns.current).forEach((id) => {
      conns.current[id]?.close();
    });
    socket.emit('stopBroadcasting', account?.decodedAddress, {
      streamId,
    });
    setStreamStatus('ended');
  };

  useEffect(() => {
    if (localVideo.current && localStream) {
      setStreamStatus('streaming');
      localVideo.current.srcObject = localStream;

      localVideo.current
        .play()
        .then((s) => s)
        .catch((err) => console.log(err));
    }
  }, [localStream]);

  useEffect(() => {
    socket.on('stopWatching', (id) => {
      try {
        camTransceiver.current?.[id]?.stop();
        delete camTransceiver.current?.[id];

        micTransceiver.current?.[id]?.stop();
        delete micTransceiver.current?.[id];

        scrAudioTransceiver.current?.[id]?.stop();
        delete scrAudioTransceiver.current?.[id];

        scrCaptureTransceiver.current?.[id]?.stop();
        delete scrCaptureTransceiver.current?.[id];

        conns.current?.[id]?.close();
        delete conns.current?.[id];
      } catch {
        console.log('error when stop a peer connection');
      }
    });
  }, [socket]);

  const handlePlayerReady = (player: HTMLVideoElement) => {
    localVideo.current = player;
  };

  return (
    <div className={cx(styles.layout)}>
      <Player
        onReady={handlePlayerReady}
        mode="broadcast"
        isMuted={isSoundMuted}
        onSoundMute={handleMuteSound}
        isCameraBlocked={Boolean(streamType === 'camera' && isCameraBlocked)}
        onCameraBlock={handleBlockCamera}
        onStopStream={handleStopStream}
        isSharingScreen={streamType === 'screen'}
        onShareScreen={handleScreenShare}
      />
      {streamStatus === 'loading' && <div className={cx(styles['start-stream-curtain'])}>loading...</div>}
      {streamStatus === 'not-started' && (
        <div className={cx(styles['start-stream-curtain'])}>
          <Button variant="primary" label="Start Stream" icon={StreamSignalSVG} onClick={startStream} />
        </div>
      )}
      {streamStatus === 'ended' && (
        <div className={cx(styles['start-stream-curtain'])}>
          <h3>You&apos;ve ended the stream</h3>
          <Button variant="primary" label="Repeat" icon={StreamSignalSVG} onClick={startStream} />
          <Button variant="outline" label="Close" onClick={handleGoToAccountPage} />
        </div>
      )}
      {streamStatus === 'already-have' && (
        <div className={cx(styles['start-stream-curtain'])}>
          <h3>You already have a stream running</h3>
        </div>
      )}
    </div>
  );
}

export { Broadcast };

import { useAccount } from '@gear-js/react-hooks';
import { MutableRefObject, useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { useGetStateQuery } from '@/app/utils';
import StreamSignalSVG from '@/assets/icons/signal-stream-icon.svg';
import { ADDRESS } from '@/consts';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { RTC_CONFIG } from '../../config';
import { TrackIds } from '../../types';
import { Player } from '../Player';

import { BroadcastProps, AnswerMsg, CandidateMsg, WatchMsg, StreamStatus, StreamType } from './Broadcast.interface';
import styles from './Broadcast.module.scss';

function Broadcast({ socket, streamId }: BroadcastProps) {
  const { account } = useAccount();
  const navigate = useNavigate();
  const { streams } = useGetStateQuery();

  const localVideo: MutableRefObject<HTMLVideoElement | null> = useRef(null);
  const conns: MutableRefObject<Record<string, RTCPeerConnection>> = useRef({});
  const commonStream: MutableRefObject<MediaStream> = useRef(new MediaStream());

  const trackIds: MutableRefObject<TrackIds> = useRef({
    microphone: null,
    camera: null,
    screenSound: null,
    screenCapture: null,
  });

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

      const microphoneId = trackIds.current.microphone;

      if (microphoneId && commonStream.current.getTrackById(microphoneId)) {
        commonStream.current.removeTrack(commonStream.current.getTrackById(microphoneId) as MediaStreamTrack);
        trackIds.current.microphone = null;
      }

      Object.keys(conns.current).forEach((id) => {
        micTransceiver.current[id]?.stop();
      });

      const cameraId = trackIds.current.camera;

      if (cameraId && commonStream.current.getTrackById(cameraId)) {
        commonStream.current.removeTrack(commonStream.current.getTrackById(cameraId) as MediaStreamTrack);
        trackIds.current.camera = null;
      }

      Object.keys(conns.current).forEach((id) => {
        camTransceiver.current[id]?.stop();
      });

      //sets new ones
      const micTrack = requestedStream.getAudioTracks()?.[0];

      if (micTrack) {
        commonStream.current.addTrack(micTrack);
        trackIds.current.microphone = micTrack.id;

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
        trackIds.current.camera = camTrack.id;

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
      //we need just remove their ids

      trackIds.current.microphone = null;
      trackIds.current.camera = null;
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

    const micTrack = requestedStream.getAudioTracks()?.[0];

    if (micTrack) {
      commonStream.current.addTrack(micTrack);
      trackIds.current.microphone = micTrack.id;
    } else {
      setIsSoundMuted(true);
    }

    const camTrack = requestedStream.getVideoTracks()?.[0];

    if (camTrack) {
      commonStream.current.addTrack(camTrack);
      trackIds.current.camera = camTrack.id;
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

        //replaces camera remote track to null
        const idOfCameraTrack = trackIds.current.camera;
        if (idOfCameraTrack) {
          Object.keys(conns.current).forEach((id) => {
            if (camTransceiver.current[id]) {
              camTransceiver.current[id]?.stop();
            }
          });
          (commonStream.current.getTrackById(idOfCameraTrack) as MediaStreamTrack).enabled = false;
        }

        //adds or replaces screenSound remote tracks to value
        const requestedScreenAudioTrack = screenStream.getAudioTracks()?.[0];
        const idOfExistingScreenAudioTrack = trackIds.current.screenSound;

        if (!idOfExistingScreenAudioTrack && requestedScreenAudioTrack) {
          trackIds.current.screenSound = requestedScreenAudioTrack.id;

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
        const idOfExistingScreenCaptureTrack = trackIds.current.screenCapture;

        if (!idOfExistingScreenCaptureTrack && requestedScreenCaptureTrack) {
          trackIds.current.screenCapture = requestedScreenCaptureTrack.id;

          commonStream.current.addTrack(requestedScreenCaptureTrack);
          Object.keys(conns.current).forEach((id) => {
            scrCaptureTransceiver.current[id] = conns.current[id]?.addTransceiver(requestedScreenCaptureTrack, {
              direction: 'sendonly',
              streams: [commonStream.current],
            });
          });
        }

        //creates new local stream

        const ids = [trackIds.current.microphone, trackIds.current.screenSound, trackIds.current.screenCapture];

        setLocalStream(
          () =>
            new MediaStream(
              ids.filter((id) => !!id).map((id) => commonStream.current.getTrackById(id as string) as MediaStreamTrack),
            ),
        );

        screenStream.getTracks()[0].onended = () => {
          //replacing screenSound and screenCapture remote tracks to null
          const audId = trackIds.current.screenSound;
          if (audId) {
            Object.keys(conns.current).forEach((id) => {
              scrAudioTransceiver.current[id]?.stop();
              scrAudioTransceiver.current[id] = null;
            });
            commonStream.current.removeTrack(commonStream.current.getTrackById(audId) as MediaStreamTrack);
            trackIds.current.screenSound = null;
          }

          const capId = trackIds.current.screenCapture;
          if (capId) {
            Object.keys(conns.current).forEach((id) => {
              scrCaptureTransceiver.current[id]?.stop();
              scrCaptureTransceiver.current[id] = null;
            });
            commonStream.current.removeTrack(commonStream.current.getTrackById(capId) as MediaStreamTrack);
            trackIds.current.screenCapture = null;
          }

          //replacing camera remote track to value
          const idOfCameraTrackNew = trackIds.current.camera;

          if (idOfCameraTrackNew) {
            (commonStream.current.getTrackById(idOfCameraTrackNew) as MediaStreamTrack).enabled = true;

            Object.keys(conns.current).forEach((id) => {
              camTransceiver.current[id] = conns.current[id].addTransceiver(
                commonStream.current.getTrackById(idOfCameraTrackNew) as MediaStreamTrack,
                {
                  direction: 'sendonly',
                  streams: [commonStream.current],
                },
              );
            });
          }

          const newRequiredId = [trackIds.current.microphone, trackIds.current.camera];
          setLocalStream(
            () =>
              new MediaStream(
                newRequiredId
                  .filter((id) => !!id)
                  .map((id) => commonStream.current.getTrackById(id as string) as MediaStreamTrack),
              ),
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
    const idOfMicrophone = trackIds.current.microphone;

    if (isMuted) {
      if (idOfMicrophone) {
        Object.keys(conns.current).forEach((id) => {
          const transceiver = micTransceiver.current[id];

          if (transceiver?.sender.track) {
            transceiver.sender.track.enabled = true;
          }
        });
        (commonStream.current.getTrackById(idOfMicrophone) as MediaStreamTrack).enabled = true;
        setIsSoundMuted(() => false);
      }
    }
    if (!isMuted) {
      if (idOfMicrophone) {
        Object.keys(conns.current).forEach((id) => {
          const transceiver = micTransceiver.current[id];

          if (transceiver?.sender.track) {
            transceiver.sender.track.enabled = false;
          }
        });
        (commonStream.current.getTrackById(idOfMicrophone) as MediaStreamTrack).enabled = false;
        setIsSoundMuted(() => true);
      }
    }
  };

  const handleBlockCamera = (isBlocked: boolean) => {
    if (streamType === 'camera') {
      const idOfCamera = trackIds.current.camera;

      if (isBlocked) {
        if (idOfCamera) {
          Object.keys(conns.current).forEach((id) => {
            const transceiver = camTransceiver.current[id];

            if (transceiver?.sender.track) {
              transceiver.sender.track.enabled = true;
            }
          });
          (commonStream.current.getTrackById(idOfCamera) as MediaStreamTrack).enabled = true;
        }
      }

      if (!isBlocked) {
        if (idOfCamera) {
          Object.keys(conns.current).forEach((id) => {
            const transceiver = camTransceiver.current[id];

            if (transceiver?.sender.track) {
              transceiver.sender.track.enabled = false;
            }
          });
          (commonStream.current.getTrackById(idOfCamera) as MediaStreamTrack).enabled = false;
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
      await loadDevices();

      socket.emit('broadcast', account?.decodedAddress, { streamId });

      socket.on('watch', (idOfWatcher: string, msg: WatchMsg) => {
        conns.current[idOfWatcher] = new RTCPeerConnection(RTC_CONFIG);

        const micId = trackIds.current.microphone;

        if (micId) {
          micTransceiver.current[idOfWatcher] = conns.current[idOfWatcher]?.addTransceiver(
            commonStream.current.getTrackById(micId) as MediaStreamTrack,
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        const camId = trackIds.current.camera;

        if (camId) {
          camTransceiver.current[idOfWatcher] = conns.current[idOfWatcher]?.addTransceiver(
            commonStream.current.getTrackById(camId) as MediaStreamTrack,
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        const scrSoundId = trackIds.current.screenSound;

        if (scrSoundId) {
          scrAudioTransceiver.current[idOfWatcher] = conns.current[idOfWatcher].addTransceiver(
            commonStream.current.getTrackById(scrSoundId) as MediaStreamTrack,
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        const scrCaptureId = trackIds.current.screenCapture;

        if (scrCaptureId) {
          scrAudioTransceiver.current[idOfWatcher] = conns.current[idOfWatcher].addTransceiver(
            commonStream.current.getTrackById(scrCaptureId) as MediaStreamTrack,
            {
              direction: 'sendonly',
              streams: [commonStream.current],
            },
          );
        }

        conns.current[idOfWatcher].onicecandidate = (event: RTCPeerConnectionIceEvent) => {
          if (event.candidate) {
            socket.emit('candidate', idOfWatcher, { userId: account.address, candidate: event.candidate, streamId });
          }
        };

        conns.current[idOfWatcher].onnegotiationneeded = () => {
          conns.current[idOfWatcher]
            .createOffer()
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
          {streams?.[streamId]?.img_link && (
            <>
              <img
                src={streams?.[streamId]?.img_link}
                alt="stream background"
                className={cx(styles['stream-background'])}
              />
              <div className={cx(styles['backdrop-filter'])} />
            </>
          )}
          <Button
            variant="primary"
            label="Start Stream"
            icon={StreamSignalSVG}
            className={cx(styles['start-stream-button'])}
            onClick={startStream}
          />
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

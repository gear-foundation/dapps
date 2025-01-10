import { Server, Socket } from 'socket.io';
import cors from 'cors';
import { createServer } from 'http';
import express from 'express';

import { isUserSubscribed } from './contract';
import { isValidSig } from './helpers';
import {
  IAnswerMsg,
  IBroadcastMsg,
  ICandidateMsg,
  IOfferMsg,
  IStopBroadcastingMsg,
  IWatchMsg,
  IStopWatchingMsg,
  GetInfoForUserMsg,
} from '../types';
import { HexString } from '@gear-js/api';

const app = express();
app.use(cors());
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  next();
});

export const server = createServer(app);

const io = new Server(server, {
  cors: {
    origin: '*',
  },
});

const connections = new Map<string, { [userId: string]: Socket }>(); //just all connections by streams (not depending on whether stream goes or not)
const streams = new Map<string, string>(); //{streamId: broadcasterId} - just streams with broadcaster id by stream id

io.sockets.on('error', err => {
  console.error(err);
});

io.on('connection', socket => {
  socket.on(
    'broadcast',
    (broadcasterId: string, { streamId }: IBroadcastMsg) => {
      for (let streamKey of streams.keys()) {
        if (streams.get(streamKey) === broadcasterId) {
          return socket.emit('error', {
            message: `You already have a stream running`,
          });
        }
      }

      const neededStreamConnections = connections.get(streamId);

      if (neededStreamConnections) {
        //if we have connections by required stream
        neededStreamConnections[broadcasterId] = socket; //then set broadcaster socket to this connection
      } else {
        connections.set(streamId, { [broadcasterId]: socket }); //else create connections list by the stream and sets broadcaster socket
      }

      streams.set(streamId, broadcasterId); //set new stream with broadcaster user id

      const updatedStreamConnections = connections.get(streamId);

      if (updatedStreamConnections) {
        Object.keys(updatedStreamConnections).forEach(
          (
            userId //emits to all users with current stream connection that stream has started
          ) =>
            updatedStreamConnections[userId].emit(
              'isStreaming',
              !!streams.get(streamId)
            )
        );
      }
    }
  );

  socket.on(
    'watch',
    async (userId: HexString, { encodedId, signedMsg, streamId }: IWatchMsg) => {
      if (!isValidSig(encodedId, signedMsg)) {
        return socket.emit('error', { message: `Signature isn't valid` }); //check if sign is valid
      }

      if (!streams.has(streamId)) {
        return socket.emit('error', {
          message: `Stream with id ${streamId} hasn't started yet`, //check if stream has started
        });
      }

      const broadcasterId = streams.get(streamId) as string;

      if (!(await isUserSubscribed(broadcasterId, userId))) {
        return socket.emit('error', {
          message: `You aren't subscribed to this speaker`, //check if user is subscribed
        });
      }

      const neededStreamConnections = connections.get(streamId); //searches for connection by required stream

      if (neededStreamConnections) {
        //if we have one
        neededStreamConnections[broadcasterId]?.emit('watch', userId, {
          //then emit our broadcaster about new watcher
          streamId,
        });
        neededStreamConnections[userId] = socket; //and adds socket of watcher to our connection
      } else {
        connections.set(streamId, { [userId]: socket }); //or sets a new connection by the socket
      }

      const updatedStreamConnections = connections.get(streamId);

      if (updatedStreamConnections) {
        // just another check on existing
        const updatedStreamConnectionsKeys = Object.keys(
          updatedStreamConnections
        );

        updatedStreamConnectionsKeys.forEach(userId => {
          updatedStreamConnections[userId].emit(
            'watchersCount',
            updatedStreamConnectionsKeys.length - 1 //watchers are all users but not broadcaster
          );

          updatedStreamConnections[userId].emit(
            'isStreaming',
            !!streams.get(streamId)
          );
        });
      }
    }
  );

  socket.on(
    'stopBroadcasting',
    (broadcasterId, { streamId }: IStopBroadcastingMsg) => {
      const neededStreamBroadcasterId = streams.get(streamId); // gets broadcaster id from streams Map

      if (neededStreamBroadcasterId) {
        // if streaming
        const neededStreamConnections = connections.get(streamId); //then get all connections by stream

        if (neededStreamConnections) {
          //if connections by stream object exists

          Object.keys(neededStreamConnections).forEach(userId => {
            //then iterate over all connections and emits not streaming
            neededStreamConnections[userId].emit('isStreaming', false);

            if (userId !== broadcasterId) {
              neededStreamConnections[userId].emit(
                'stopBroadcasting',
                broadcasterId,
                {
                  streamId,
                  userId,
                }
              );
            }
          });

          delete neededStreamConnections[neededStreamBroadcasterId]; //removes broadcaster from connections
        }

        streams.delete(streamId); //removes stream from currently streamed
      }
    }
  );

  socket.on(
    'offer',
    (broadcasterId, { streamId, userId, description }: IOfferMsg) => {
      const userConnection = connections.get(streamId)?.[userId];

      if (userConnection) {
        userConnection!.emit('offer', broadcasterId, {
          streamId,
          userId,
          description,
        });
      }
    }
  );

  socket.on(
    'answer',
    (broadcasterId, { streamId, userId, description }: IAnswerMsg) => {
      const broadcasterConnection = connections.get(streamId)?.[broadcasterId];

      if (broadcasterConnection) {
        broadcasterConnection.emit('answer', broadcasterId, {
          streamId,
          userId,
          description,
        });
      }
    }
  );

  socket.on(
    'candidate',
    (broadcasterId, { userId, streamId, candidate }: ICandidateMsg) => {
      const broadcasterConnection = connections.get(streamId)?.[broadcasterId];

      if (broadcasterConnection) {
        broadcasterConnection.emit('candidate', userId, {
          candidate,
        });
      }
    }
  );

  socket.on('stopWatching', (userId, { streamId }: IStopWatchingMsg) => {
    const neededStreamBroadcasterId = streams.get(streamId);

    if (neededStreamBroadcasterId) {
      //if streaming now
      const broadcasterConnection =
        connections.get(streamId)?.[neededStreamBroadcasterId]; //then get connection of broadcaster
      broadcasterConnection?.emit('stopWatching', userId, { streamId }); //and emits to broadcaster that a user has stopped watching
    }

    const connectionsByStream = connections.get(streamId); //gets connections by the stream

    if (connectionsByStream) {
      delete connectionsByStream[userId]; //deletes user connection from stream connections

      const connectionsByStreamLength = Object.keys(connectionsByStream).length; //detecting how many connections has left: ;

      if (!connectionsByStreamLength) {
        //if there're no connections for stream
        connections.delete(streamId); //then deletes connections object for the stream
      } else {
        Object.keys(connectionsByStream).forEach(
          (
            userId //else emits to left users count of all current stream connections
          ) =>
            connectionsByStream[userId].emit(
              'watchersCount',
              connectionsByStreamLength - 1
            )
        );
      }
    }
  });

  socket.on('getWatchersCount', (id, { streamId }: GetInfoForUserMsg) => {
    const connectionsByStream = connections.get(streamId);
    const connectionsByStreamKeys = Object.keys(connectionsByStream || {});

    if (connectionsByStream && connectionsByStreamKeys.length) {
      connectionsByStreamKeys.forEach(userId =>
        connectionsByStream[userId].emit(
          'watchersCount',
          connectionsByStreamKeys.length - 1
        )
      );
    }
  });

  socket.on('getIsStreaming', (id, msg: GetInfoForUserMsg) => {
    socket.emit('isStreaming', !!streams.get(msg.streamId));
  });

  socket.on('disconnect', r => {
    console.log('CLOSE', r);

    for (let streamId of connections.keys()) {
      //iterate over the connections
      const connectionsByStream = connections.get(streamId); // get stream connections by stream id

      const searchedConnection = Object.keys(connectionsByStream || {}).find(
        //finds needed connection in stream connections
        userId => connectionsByStream?.[userId].id === socket.id
      );

      if (searchedConnection && connectionsByStream) {
        //if they exist
        for (let streamId of streams.keys()) {
          //then iterates over streams
          if (streams.get(streamId) === searchedConnection) {
            //if streams has our connection
            streams.delete(streamId); //then delete it from streams

            Object.keys(connectionsByStream).forEach(userId => {
              //iterates over stream connections
              connectionsByStream?.[userId].emit('isStreaming', false); //emits isStreaming false to all connections

              if (userId !== searchedConnection) {
                // emits stop broadcasting for all watchers
                connectionsByStream?.[userId].emit(
                  'stopBroadcasting',
                  searchedConnection,
                  {
                    streamId,
                    userId,
                  }
                );
              }
            });
          } else {
            //if streams doen't have our connection means it's watcher

            Object.keys(connectionsByStream).forEach(userId => {
              //iterates over stream connections and updates watchers count
              connectionsByStream?.[userId].emit(
                'watchersCount',
                Object.keys(connectionsByStream).length - 2
              );
            });
          }
        }

        if (Object.keys(connectionsByStream || {}).length <= 1) {
          //removes the connection from connections
          connections.delete(streamId);
        } else {
          delete connectionsByStream?.[searchedConnection];
        }
      }
    }
  });
});

app.get('/is-already-having-stream', (req, res) => {
  const userId = req.query.address;

  for (let streamKey of streams.keys()) {
    if (streams.get(streamKey) === userId) {
      return res.send(true);
    }
  }

  res.send(false);
});

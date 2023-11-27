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
} from 'types';

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

const connections = new Map<string, Socket>();
const streams = new Map<string, string>();
const connectionsPerStream = new Map<
  string,
  { count: number; broadcasterId: string }
>();

io.sockets.on('error', err => {
  console.error(err);
});

io.on('connection', socket => {
  socket.on('broadcast', (id: string, msg: IBroadcastMsg) => {
    connections.set(id, socket);
    streams.set(msg.streamId, id);

    if (!connectionsPerStream.has(msg.streamId)) {
      connectionsPerStream.set(msg.streamId, { count: 0, broadcasterId: '' });
    }
    connectionsPerStream.get(msg.streamId)!.broadcasterId = id;

    for (let connection of connections.keys()) {
      connections
        .get(connection)!
        .emit('isStreaming', !!streams.get(msg.streamId));
    }
  });

  socket.on('watch', async (id: string, msg: IWatchMsg) => {
    if (!isValidSig(msg.encodedId, msg.signedMsg)) {
      return socket.emit('error', { message: `Signature isn't valid` });
    }

    if (!streams.has(msg.streamId)) {
      return socket.emit('error', {
        message: `Stream with id ${msg.streamId} hasn't started yet`,
      });
    }

    const broadcasterId = streams.get(msg.streamId) as string;

    if (!(await isUserSubscribed(broadcasterId, id))) {
      return socket.emit('error', {
        message: `You aren't subscribed to this speaker`,
      });
    }

    connections.get(broadcasterId)?.emit('watch', id, msg);

    connections.set(id, socket);

    connectionsPerStream.get(msg.streamId)!.count++;

    for (let connection of connections.keys()) {
      connections
        .get(connection)!
        .emit('watchersCount', connectionsPerStream.get(msg.streamId)!.count);

      connections
        .get(connection)!
        .emit('isStreaming', !!streams.get(msg.streamId));
    }
  });

  socket.on('stopBroadcasting', (broadcasterId, msg: IStopBroadcastingMsg) => {
    if (streams.has(msg.streamId)) {
      for (let connection of connections.keys()) {
        connections.get(connection)!.emit('isStreaming', false);
        if (connection !== broadcasterId) {
          connections.get(connection)!.emit('stopBroadcasting', broadcasterId, {
            streamId: msg.streamId,
            watcherId: connection,
          });
        }
      }

      connections.delete(streams.get(msg.streamId)!);
      connectionsPerStream.delete(msg.streamId);
      streams.delete(msg.streamId);
    }
  });

  socket.on('offer', (id, msg: IOfferMsg) => {
    if (connections.has(msg.userId)) {
      console.log(msg.userId);
      connections.get(msg.userId)!.emit('offer', id, msg);
    }
  });

  socket.on('answer', (id, msg: IAnswerMsg) => {
    if (connections.has(id)) {
      connections.get(id)?.emit('answer', id, msg);
    }
  });

  socket.on('candidate', (id, msg: ICandidateMsg) => {
    if (connections.has(id)) {
      connections.get(id)?.emit('candidate', msg.id, msg);
    }
  });

  socket.on('stopWatching', (id, msg: IStopWatchingMsg) => {
    if (streams.get(msg.streamId)) {
      const broadcasterId = streams.get(msg.streamId) as string;

      connections.get(broadcasterId)?.emit('stopWatching', id, msg);
      connections.delete(id);
    }
    if (
      connectionsPerStream.get(msg.streamId) &&
      connectionsPerStream.get(msg.streamId)!.count > 0
    ) {
      connectionsPerStream.get(msg.streamId)!.count--;

      for (let connection of connections.keys()) {
        connections
          .get(connection)!
          .emit('watchersCount', connectionsPerStream.get(msg.streamId)!.count);
      }
    }
  });

  socket.on('getWatchersCount', (id, msg: GetInfoForUserMsg) => {
    if (connectionsPerStream.get(msg.streamId)) {
      socket.emit(
        'watchersCount',
        connectionsPerStream.get(msg.streamId)!.count
      );
    }
  });

  socket.on('getIsStreaming', (id, msg: GetInfoForUserMsg) => {
    socket.emit('isStreaming', !!streams.get(msg.streamId));
  });

  socket.on('disconnect', r => {
    console.log('CLOSE', r);
  });
});

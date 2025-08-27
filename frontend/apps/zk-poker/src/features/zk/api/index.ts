import {
  SubscribeToTasksPayload,
  SubmitResultPayload,
  UnsubscribeFromTasksPayload,
  ZkTaskResponse,
  SubscriptionStatusResponse,
  ResultProcessedResponse,
  WebSocketError,
} from './types';
import { zkWebSocketService, WebSocketEventHandlers } from './websocket-service';

// WebSocket-based API functions
export const subscribeToTasks = async (payload: SubscribeToTasksPayload): Promise<void> => {
  if (!payload?.lobbyAddress || !payload?.playerAddress) {
    throw new Error('Lobby address or player address is not defined');
  }

  try {
    await zkWebSocketService.subscribeToTasks(payload);
  } catch (error) {
    console.error('Failed to subscribe to tasks:', error);
    throw new Error(error instanceof Error ? error.message : 'Failed to subscribe to tasks');
  }
};

export const submitResult = async (payload: SubmitResultPayload): Promise<void> => {
  if (!payload?.lobbyAddress || !payload?.playerAddress) {
    throw new Error('Lobby address or player address is not defined');
  }

  try {
    await zkWebSocketService.submitResult(payload);
  } catch (error) {
    console.error('Failed to submit result:', error);
    throw new Error(error instanceof Error ? error.message : 'Failed to submit result');
  }
};

export const unsubscribeFromTasks = async (payload: UnsubscribeFromTasksPayload): Promise<void> => {
  if (!payload?.lobbyAddress || !payload?.playerAddress) {
    throw new Error('Lobby address or player address is not defined');
  }

  try {
    await zkWebSocketService.unsubscribeFromTasks(payload);
  } catch (error) {
    console.error('Failed to unsubscribe from tasks:', error);
    throw new Error(error instanceof Error ? error.message : 'Failed to unsubscribe from tasks');
  }
};

export const setWebSocketEventHandlers = (handlers: WebSocketEventHandlers): void => {
  zkWebSocketService.setEventHandlers(handlers);
};

export const isWebSocketConnected = (): boolean => {
  return zkWebSocketService.isConnected();
};

export const reconnectWebSocket = (): void => {
  zkWebSocketService.reconnect();
};

export const disconnectWebSocket = (): void => {
  zkWebSocketService.disconnect();
};

// Export types for convenience
export type {
  WebSocketEventHandlers,
  SubscribeToTasksPayload,
  SubmitResultPayload,
  UnsubscribeFromTasksPayload,
  ZkTaskResponse,
  SubscriptionStatusResponse,
  ResultProcessedResponse,
  WebSocketError,
};

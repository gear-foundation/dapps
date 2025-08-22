import { io, Socket } from 'socket.io-client';

import { ENV } from '@/app/consts';

import {
  SubscribeToTasksPayload,
  SubmitResultPayload,
  UnsubscribeFromTasksPayload,
  ZkTaskResponse,
  SubscriptionStatusResponse,
  ResultProcessedResponse,
  WebSocketError,
  GameProgressEvent,
} from './types';

export type WebSocketEventHandlers = {
  onNewTask?: (task: ZkTaskResponse) => void;
  onSubscriptionStatus?: (status: SubscriptionStatusResponse) => void;
  onResultProcessed?: (response: ResultProcessedResponse) => void;
  onError?: (error: WebSocketError) => void;
  onConnect?: () => void;
  onDisconnect?: () => void;
  onGameProgress?: (progress: GameProgressEvent) => void;
};

class ZkWebSocketService {
  private socket: Socket | null = null;
  private handlers: WebSocketEventHandlers = {};
  private isConnecting = false;

  constructor() {
    this.connect();
  }

  private connect(): void {
    if (this.socket?.connected || this.isConnecting) {
      return;
    }

    this.isConnecting = true;

    try {
      this.socket = io(ENV.ZK_POKER_BACKEND, {
        transports: ['websocket', 'polling'],
        autoConnect: true,
        reconnection: true,
        reconnectionAttempts: 5,
        reconnectionDelay: 3000,
      });

      this.setupEventListeners();
    } catch (error) {
      console.error('Failed to create socket connection:', error);
      this.isConnecting = false;
    }
  }

  private setupEventListeners(): void {
    if (!this.socket) return;

    this.socket.on('connect', () => {
      console.log('WebSocket connected');
      this.isConnecting = false;
      this.handlers.onConnect?.();
    });

    this.socket.on('disconnect', (reason) => {
      console.log('WebSocket disconnected:', reason);
      this.isConnecting = false;
      this.handlers.onDisconnect?.();
    });

    this.socket.on('newTask', (task: ZkTaskResponse) => {
      console.log('Received new task:', task);
      this.handlers.onNewTask?.(task);
    });

    this.socket.on('gameProgress', (progress: GameProgressEvent) => {
      console.log('Received game progress:', progress);
      this.handlers.onGameProgress?.(progress);
    });

    this.socket.on('subscriptionStatus', (status: SubscriptionStatusResponse) => {
      console.log('Subscription status:', status);
      this.handlers.onSubscriptionStatus?.(status);
    });

    this.socket.on('resultProcessed', (response: ResultProcessedResponse) => {
      console.log('Result processed:', response);
      this.handlers.onResultProcessed?.(response);
    });

    this.socket.on('error', (error: WebSocketError) => {
      console.error('WebSocket error:', error);
      this.handlers.onError?.(error);
    });

    this.socket.on('connect_error', (error) => {
      console.error('WebSocket connection error:', error);
      this.isConnecting = false;
      this.handlers.onError?.({
        message: error.message || 'Connection failed',
        code: 'CONNECTION_ERROR',
      });
    });
  }

  public setEventHandlers(handlers: WebSocketEventHandlers): void {
    this.handlers = { ...this.handlers, ...handlers };
  }

  public subscribeToTasks(payload: SubscribeToTasksPayload): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.socket?.connected) {
        reject(new Error('WebSocket not connected'));
        return;
      }

      try {
        this.socket.emit('subscribeToTasks', payload);
        resolve();
      } catch (error) {
        reject(error as Error);
      }
    });
  }

  public submitResult(payload: SubmitResultPayload): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.socket?.connected) {
        reject(new Error('WebSocket not connected'));
        return;
      }

      try {
        this.socket.emit('submitResult', payload);
        resolve();
      } catch (error) {
        reject(error as Error);
      }
    });
  }

  public unsubscribeFromTasks(payload: UnsubscribeFromTasksPayload): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!this.socket?.connected) {
        reject(new Error('WebSocket not connected'));
        return;
      }

      try {
        this.socket.emit('unsubscribeFromTasks', payload);
        resolve();
      } catch (error) {
        reject(error as Error);
      }
    });
  }

  public isConnected(): boolean {
    return this.socket?.connected || false;
  }

  public disconnect(): void {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
    this.handlers = {};
  }

  public reconnect(): void {
    this.disconnect();
    this.connect();
  }
}

export const zkWebSocketService = new ZkWebSocketService();

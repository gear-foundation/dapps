import { Mark } from '@/app/utils';

export type Cell = Mark | null;

export type IGameCountdown = { isActive: boolean; value: number } | undefined;

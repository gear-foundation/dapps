import { WALLET } from './consts';
import type { SVGComponent } from '@/app/types';

export type IWalletId = keyof typeof WALLET;

export type IWalletExtensionContent = { name: string; SVG: SVGComponent };

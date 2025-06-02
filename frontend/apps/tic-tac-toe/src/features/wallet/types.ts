import type { SVGComponent } from '@/app/types';

import { WALLET } from './consts';

export type IWalletId = keyof typeof WALLET;

export type IWalletExtensionContent = { name: string; SVG: SVGComponent };

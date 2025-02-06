import type { SVGComponent } from '../../types';

import { WALLET } from './consts';

export type IWalletId = keyof typeof WALLET;

export type IWalletExtensionContent = { name: string; SVG: SVGComponent };

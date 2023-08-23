import { isHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';

const copyToClipboard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

const isProgramIdValid = (value: string): value is HexString => isHex(value, 256);

export { copyToClipboard, isProgramIdValid };

import { isNumber } from '@polkadot/util';

function isNumeric(value: string) {
  return /^-?\d+$/.test(value);
}
export const isExists = (value: string) => (!value ? 'Field is required' : null);
export const isNumberValue = (value: string) => (!isNumeric(value) ? 'String must be number' : null);

export const numberRequired = (value: string) =>
  !value ? 'Field is required' : !isNumeric(value) ? 'String must be number' : null;

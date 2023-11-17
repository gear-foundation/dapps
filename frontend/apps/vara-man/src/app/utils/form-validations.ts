import { isHex } from '@polkadot/util';

export const isExists = (value: string) => (!value ? 'Field is required' : null);
export const isHexValue = (value: string) => (!isHex(value) ? 'String must be in Hex format' : null);

export const hexRequired = (value: string) =>
  !value ? 'Field is required' : !isHex(value) ? 'String must be in Hex format' : null;

export const validateLength = (value: string, minLength: number, maxLength: number) => {
  if (value.length < minLength) {
    return `Field should be at least ${minLength} characters long`;
  }
  if (value.length > maxLength) {
    return `Field should be at most ${maxLength} characters long`;
  }
  return null;
};

export const containsValidCharacters = (value: string) => {
  const validCharactersRegex = /^[a-zA-Z0-9]*$/;

  if (!validCharactersRegex.test(value)) {
    return 'Field should contain only letters (a-z) and digits (0-9)';
  }

  return null;
};

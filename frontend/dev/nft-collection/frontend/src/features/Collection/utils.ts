import moment from 'moment';
import { Token } from './types';

export const getNotMintedTokens = (tokens: Token[]) => tokens.filter((token) => !token.owner);

export const getTimeFormatFromStateDate = (dateString: string) => {
  const date = moment(dateString, 'YYYY-M-D HH:mm');
  const now = moment();

  const diff = now.diff(date, 'seconds');

  if (diff < 60) {
    return `${Math.floor(diff)} seconds ago`;
  }
  if (diff < 3600) {
    return `${Math.floor(diff / 60)} ${Math.floor(diff / 60) > 1 ? 'minutes' : 'minute'} ago`;
  }
  if (diff < 86400) {
    return `${Math.floor(diff / 3600)} ${Math.floor(diff / 3600) > 1 ? 'hours' : 'hour'} ago`;
  }

  return `${Math.floor(diff / 86400)} ${Math.floor(diff / 86400) > 1 ? 'days' : 'day'} ago`;
};

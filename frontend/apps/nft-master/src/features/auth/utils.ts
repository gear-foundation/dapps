import { AnyJson } from '@polkadot/types/types';
import { AUTH_API_ADDRESS } from './consts';

export function trimEndSlash(url: string): string {
  return url?.endsWith('/') ? url.slice(0, -1) : url;
}

export const API_URL = trimEndSlash(AUTH_API_ADDRESS);

const post = (url: string, payload: AnyJson) =>
  fetch(`${API_URL}/${url}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });

const fetchAuth = <T>(url: string, method: string, token: string) =>
  fetch(`${API_URL}/${url}`, {
    method,
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  }).then((response) => {
    if (!response.ok) throw new Error(response.statusText);

    return response.json() as T;
  });

export { post, fetchAuth };

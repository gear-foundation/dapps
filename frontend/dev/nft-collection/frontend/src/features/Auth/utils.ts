import { AnyJson } from '@polkadot/types/types';
import { AUTH_API_ADDRESS } from './consts';
import { IApiError } from './types';

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

const fetchAuth = <T>(url: string, method: string, token: string, payload?: AnyJson) =>
  fetch(`${API_URL}/${url}`, {
    method,
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: payload ? JSON.stringify(payload) : undefined,
  }).then(async (response) => {
    const json = await response.json();
    if (!response.ok) throw new Error(await (json as IApiError).message);
    return json as T;
  });

export { post, fetchAuth };

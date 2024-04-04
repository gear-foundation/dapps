import { HexString } from '@gear-js/api';

async function guardedFetch<T extends object>(...args: Parameters<typeof fetch>) {
  const response = await fetch(...args);

  if (!response.ok) throw new Error(response.statusText);

  const result = (await response.json()) as T | { error: string };

  if ('error' in result) throw new Error(result.error);

  return result;
}

async function getVoucherId(backend: string, account: string, program: HexString): Promise<`0x${string}`> {
  const url = `${backend}gasless/voucher/request`;
  const method = 'POST';
  const headers = { 'Content-Type': 'application/json' };
  const body = JSON.stringify({ account, program });

  try {
    const { voucherId } = await guardedFetch<{ voucherId: HexString }>(url, { method, headers, body });

    return voucherId;
  } catch {
    throw new Error(`Voucher couldn't be fetched`);
  }
}

async function getVoucherStatus(backend: string, program: HexString) {
  const url = `${backend}gasless/voucher/${program}/status`;

  try {
    const { enabled } = await guardedFetch<{ enabled: boolean }>(url);

    return enabled;
  } catch {
    return false;
  }
}

export { getVoucherId, getVoucherStatus };

import { getIpfsAddress } from '@/utils';

const getImageUrl = (value: string) => (value.startsWith('https://') ? value : getIpfsAddress(value));

export { getImageUrl };

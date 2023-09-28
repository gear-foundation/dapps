import clsx from 'clsx';

const copyToClipboard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

export const cx = (...styles: string[]) => clsx(...styles);

export { copyToClipboard };

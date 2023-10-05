const isGreaterThanZero = (value: string) => (+value > 0 ? null : 'Enter number');

const shortenString = (str: string): string => `${str.slice(0, 4)}...${str.slice(-4)}`;

export { isGreaterThanZero, shortenString };

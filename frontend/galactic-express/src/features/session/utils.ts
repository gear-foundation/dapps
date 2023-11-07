const isGreaterThanZero = (value: string) => (+value > 0 ? null : 'Enter number');

const shortenString = (str: string, length: number): string => `${str.slice(0, length)}...${str.slice(-length)}`;

export { isGreaterThanZero, shortenString };

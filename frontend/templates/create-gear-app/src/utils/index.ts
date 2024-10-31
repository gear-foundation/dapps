const copyToClipboard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

export { copyToClipboard };

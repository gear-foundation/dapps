const getZkLog = (logName: string, duration: number) => {
  return `[${new Date().toTimeString().split(' ')[0]}] ${logName} completed in ${(duration / 1000).toFixed(2)}s`;
};

export { getZkLog };

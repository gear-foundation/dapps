/* eslint-disable */

const logMemory = (message: string) => {
  if (typeof window !== 'undefined' && 'memory' in performance) {
    const memInfo = (performance as any).memory;
    console.log(`🧹 Memory ${message}:`, {
      used: Math.round(memInfo.usedJSHeapSize / 1024 / 1024) + 'MB',
      total: Math.round(memInfo.totalJSHeapSize / 1024 / 1024) + 'MB',
    });
  }
};

export { logMemory };

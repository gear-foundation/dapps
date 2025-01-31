const getViteEnv = (key: string) => {
  try {
    return import.meta.env[`VITE_${key}`];
  } catch (error) {
    return undefined;
  }
};

export { getViteEnv };

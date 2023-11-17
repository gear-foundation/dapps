const getCRAEnv = (key: string) => {
  try {
    return process.env[`REACT_APP_${key}`];
  } catch (error) {
    return undefined;
  }
};

const getViteEnv = (key: string) => {
  try {
    return import.meta.env[`VITE_${key}`];
  } catch (error) {
    return undefined;
  }
};

export { getCRAEnv, getViteEnv };

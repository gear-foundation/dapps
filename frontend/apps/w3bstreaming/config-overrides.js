const webpack = require('webpack');
const path = require(`path`);

const SRC = `src`;

module.exports = (config) => {
  config.plugins.push(new webpack.ProvidePlugin({ Buffer: ['buffer', 'Buffer'] }));
  config.resolve = {
    ...config.resolve,
    alias: {
      ...config.resolve.alias,
      '@': path.resolve(__dirname, `${SRC}`),
      '@ui': path.resolve(__dirname, `${SRC}/ui`),
    },
  };
  return config;
};

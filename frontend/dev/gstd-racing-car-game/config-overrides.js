const webpack = require('webpack');
const path = require(`path`);

const SRC = `src`;

module.exports = (config) => {
  config.plugins.push(new webpack.ProvidePlugin({ Buffer: ['buffer', 'Buffer'] }));
  return {
    ...config,
    plugins: [
      ...config.plugins,
      new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
      }),
    ],
    resolve: {
      ...config.resolve,
      alias: {
        '@': path.resolve(__dirname, `${SRC}`),
      },
    },
    devServer: {
      port: 3000,
    },
  };
};

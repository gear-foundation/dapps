const webpack = require('webpack');
const path = require(`path`);

const SRC = `src`;

module.exports = {
  webpack: {
    alias: {
      '@': path.resolve(__dirname, `${SRC}`),
      '@ui': path.resolve(__dirname, `${SRC}/ui`),
    },
    plugins: {
      add: [
        new webpack.ProvidePlugin({
          Buffer: ['buffer', 'Buffer'],
        }),
      ],
    },
  },
  devServer: {
    port: 3000,
  },
};

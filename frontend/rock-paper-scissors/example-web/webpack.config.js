const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: './src/functions.ts',
  mode: 'development',
  // mode: 'production',
  plugins: [
    new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
    }),
    new HtmlWebpackPlugin({
      template: "./index.html",
    })
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    fallback: {
        buffer: require.resolve('buffer/'),
    },
  },
  output: {
    library: 'Contract',
  },
  devServer: {
    static: {
      directory: path.resolve(__dirname, './contract_files'), 
      publicPath: '/contract_files'
    }
  }
};
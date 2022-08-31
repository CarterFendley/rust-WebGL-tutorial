const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
  const isProd = (args.mode === 'production');

  return {
    entry: './index.js',
    mode: isProd ? 'production' : 'development',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: isProd ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    devServer: {
      hot: false,
      liveReload: true,
      static: {
        directory: './dist',
        watch: true
      }
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, '.')
      }),
      // Below is for Edge (chrome / firefox are fine)
      new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
      })
    ]
  }
}
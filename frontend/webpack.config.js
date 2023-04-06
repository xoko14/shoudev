let path = require('path');

module.exports = {
  let webpack = require('webpack');
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, '../static')
  },
  optimization: {
    splitChunks: {
      chunks: "all",
    },
  }
}

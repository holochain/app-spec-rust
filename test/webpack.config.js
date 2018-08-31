const path = require('path');

module.exports = {
  entry: './test.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
        {
          test: /\.js$/,
          exclude: /(node_modules|bower_components)/,
          use: {
            loader: "babel-loader",
            options: {
                presets: ["@babel/preset-env"]
            }
          }
        }
    ]
  },
  stats: {
      colors: true
  },
  node: {
    fs: 'empty',
    setImmediate: false
  }
};

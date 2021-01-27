const HtmlWebpackPlugin = require('html-webpack-plugin')
const HtmlWebpackInlineSourcePlugin = require('html-webpack-inline-source-plugin');
module.exports = {
  //Inline images
  chainWebpack: config => {
    config.module
      .rule('images')
        .use('url-loader')
          .loader('url-loader')
          .tap(options => Object.assign(options, { limit: true }))
  },
  //Inline to single .html for Rust
  css: {
    extract: false,
  },
  configureWebpack: {
    optimization: {
      splitChunks: false
    },
    plugins: [
      new HtmlWebpackPlugin({
        filename: 'dist.html',
        template: 'public/index.html',
        inlineSource: '.(js|css)$'
      }),
      new HtmlWebpackInlineSourcePlugin(HtmlWebpackPlugin)
    ]
  },
}

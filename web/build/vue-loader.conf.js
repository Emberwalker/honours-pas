'use strict'
const utils = require('./utils')
const config = require('../config')
const isProduction = process.env.NODE_ENV === 'production'
const sourceMapEnabled = isProduction
  ? config.build.productionSourceMap
  : config.dev.cssSourceMap


module.exports = {
  loaders: {
    ...utils.cssLoaders({
      sourceMap: sourceMapEnabled,
      extract: isProduction
    }),
    // based on https://github.com/timeu/vue-typescript-linting/blob/master/webpack.config.js
    // 'ts': 'ts-loader!tslint-loader'
    'ts': 'ts-loader'
  },
  preLoaders: {
    // Provide options to tslint-loader, like in vue-loader's default set:
    // https://github.com/vuejs/vue-loader/blob/master/lib/loader.js#L87
    // TODO: Refactor out TSLint config so it's properly shared.
    'ts': "tslint-loader?" + JSON.stringify({
      formattersDirectory: 'formatters',
      formatter: 'PAS',
      configFile: 'tslint-vue.json',
    }),
  },
  cssSourceMap: sourceMapEnabled,
  transformToRequire: {
    video: 'src',
    source: 'src',
    img: 'src',
    image: 'xlink:href'
  }
}

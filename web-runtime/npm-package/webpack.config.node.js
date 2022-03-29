// Generated using webpack-cli https://github.com/webpack/webpack-cli

const config = require('./webpack.config.js');

module.exports = () => {
    const cfg = config();
    cfg.output.filename = 'marine-js.node.js';
    cfg.target = 'node';
    cfg.externals = ['@wasmer/wasi', '@wasmer/wasmfs'];
    return cfg;
};

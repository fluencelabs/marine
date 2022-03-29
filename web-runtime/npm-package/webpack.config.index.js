// Generated using webpack-cli https://github.com/webpack/webpack-cli

const config = require('./webpack.config.js');

module.exports = () => {
    const cfg = config();
    cfg.entry = './src/index.ts';
    cfg.output.filename = 'index.js';
    cfg.target = 'node';
    return cfg;
};

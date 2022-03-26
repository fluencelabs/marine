// Generated using webpack-cli https://github.com/webpack/webpack-cli

const config = require('./webpack.config.js');

module.exports = () => {
    const cfg = config();
    cfg.output.filename = 'marine-js.node.js';
    cfg.target = 'node';
    //
    // TODO: we want to reuse code from node_modules
    // instead of bundling AVM inside base64
    //                  ||
    //                  \/
    // cfg.externals = [
    //     {
    //         ['@fluencelabs/avm']: {
    //             root: '@fluencelabs/avm',
    //         },
    //     },
    // ];
    return cfg;
};

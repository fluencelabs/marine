// Generated using webpack-cli https://github.com/webpack/webpack-cli

const path = require('path');

// const isProduction = true;
// uncomment to debug
const isProduction = false;

const config = {
    entry: {
        index: './src/index.ts',
        background: './src/backgroundScript.ts',
    },
    output: {
        path: path.resolve('dist'),
    },
    module: {
        rules: [
            {
                test: /\.(js|ts|tsx)$/i,
                use: 'ts-loader',
                exclude: ['/node_modules/'],
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
};

module.exports = () => {
    if (isProduction) {
        config.mode = 'production';
    } else {
        config.mode = 'development';
    }

    return config;
};

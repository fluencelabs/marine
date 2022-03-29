// Generated using webpack-cli https://github.com/webpack/webpack-cli

const path = require('path');

// const isProduction = true;
// uncomment to debug
const isProduction = false;

const config = (ifDefOpts) => ({
    entry: './src/backgroundScript.ts',
    output: {
        path: path.resolve('dist'),
    },
    module: {
        rules: [
            {
                test: /\.(js|ts|tsx)$/i,
                use: [
                    // force new line
                    { loader: 'ts-loader' },
                    { loader: 'ifdef-loader', options: ifDefOpts },
                ],
                exclude: ['/node_modules/'],
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
});

module.exports = (ifDefConfig) => {
    const res = config(ifDefConfig);
    if (isProduction) {
        res.mode = 'production';
    } else {
        res.mode = 'development';
    }

    return res;
};

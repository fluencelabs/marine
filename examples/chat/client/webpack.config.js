const path = require('path');
const webpack = require('webpack');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
var HtmlWebpackPlugin = require('html-webpack-plugin');

const production = (process.env.NODE_ENV === 'production');

const config = {
    entry: {
        app: ['./src/index.ts']
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader',
                exclude: /node_modules/
            }
        ]
    },
    resolve: {
        extensions: [ '.tsx', '.ts', '.js']
    },
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'bundle'),
    },
    node: {
        fs: 'empty'
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin()
    ]
};

if (production) {
    config.mode = 'production';
} else {
    config.mode = 'development';
    config.devtool = 'inline-source-map';
    config.devServer = {
        contentBase: './bundle',
        hot: true
    };
    config.plugins = [
        ...config.plugins,
        new webpack.HotModuleReplacementPlugin()
    ];
}

module.exports = config;

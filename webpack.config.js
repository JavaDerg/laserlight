const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const DeclarationBundlerPlugin = require('declaration-bundler-webpack-plugin');

module.exports = ['source-map'].map(devtool => ({
    entry: './src_ts/index.ts',
    mode: process.env.MODE || 'production',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'laserlight.js',
        library: 'laserlight',
        libraryTarget: 'umd',
    },
    devtool,
    optimization: {
        runtimeChunk: true,
    },
    plugins: [
        new DeclarationBundlerPlugin({
            moduleName: 'laserlight',
            out: 'laserlight.d.ts',
        }),
        new WasmPackPlugin({
            crateDirectory: __dirname,
            args: ''
        }),
    ],
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: [ '.tsx', '.ts', '.js' ],
    },
}));
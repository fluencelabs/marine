import Webpack from 'webpack';
import WebpackDevServer from 'webpack-dev-server';
import webpackConfig from '../webpack.config.js';
import process from 'process';
import path from 'path';
import fs from 'fs';

// change directory to the location to the test-project.
// run all the subsequent Webpack scripts in that directory
process.chdir(path.join(__dirname, '..'));

let server;

jest.setTimeout(10000);

const startServer = async (modifyConfig?) => {
    const loadInBrowserToDebug = false;
    // const loadInBrowserToDebug = true; // use this line to debug

    modifyConfig = modifyConfig || ((_) => {});

    const cfg: any = webpackConfig();
    modifyConfig(cfg);
    const compiler = Webpack(cfg);
    const devServerOptions = { ...cfg.devServer, open: loadInBrowserToDebug };
    server = new WebpackDevServer(devServerOptions, compiler);
    await server.start();
    // wait for webpack to load
    await new Promise((resolve) => setTimeout(resolve, 1000));
};

const stopServer = async () => {
    console.log('test: stopping server');
    await server.stop();
};

const publicDir = 'public';

function copyFile(packageName: string, fileName: string) {
    const modulePath = require.resolve(packageName);
    const source = path.join(path.dirname(modulePath), fileName);
    const dest = path.join(publicDir, fileName);

    fs.copyFileSync(source, dest);
}

const copyPublicDeps = async () => {
    fs.mkdirSync(publicDir, { recursive: true });
    copyFile('@fluencelabs/marine-js', 'marine-js.web.js');
    copyFile('@fluencelabs/marine-js', 'marine-js.wasm');
    copyFile('@fluencelabs/avm', 'avm.wasm');
};

const cleanPublicDeps = async () => {
    fs.rmSync(publicDir, { recursive: true, force: true });
};

describe('Browser integration tests', () => {
    beforeEach(async () => {
        await copyPublicDeps();
    });

    afterEach(async () => {
        await stopServer();
        await cleanPublicDeps();
    });

    it('Some test', async () => {
        console.log('test: starting server...');
        await startServer();
        console.log('test: navigating to page...');
        await page.goto('http://localhost:8080/');

        console.log('test: running script in browser...');
        // @ts-ignore
        const res = await page.evaluate(async () => {
            // @ts-ignore
            return await window.MAIN();
        });

        console.log('test: checking expectations...');
        await expect(res).toMatchObject({
            retCode: 0,
            errorMessage: '',
        });
    });
});

import Webpack from 'webpack';
import WebpackDevServer from 'webpack-dev-server';
import webpackConfig from '../webpack.config.js';
import process from 'process';
import path from 'path';

// change directory to the location to the test-project.
// run all the subsequent Webpack scripts in that directory
process.chdir(path.join(__dirname, '..'));

let server;

jest.setTimeout(10000);

const startServer = async (modifyConfig?) => {
    const loadInBrowserToDebug = false; // set to true to debug

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

describe('Integration tests for web target', () => {
    afterEach(async () => {
        await stopServer();
    });

    it('AvmRunnerBackground should work correctly execute simple script"', async () => {
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

    it.skip('Should display correct error message if wasm is not served', async () => {
        console.log('test: starting server...');
        await startServer((cfg) => {
            // simulating incorrect webpack setup
            cfg.devServer.static.directory = 'public2';
        });
        console.log('test: navigating to page...');
        await page.goto('http://localhost:8080/');

        console.log('test: running script in browser...');
        // @ts-ignore
        const error = await page
            .evaluate(async () => {
                // @ts-ignore
                return await window.MAIN();
            })
            .catch((e) => e.message);

        console.log('test: checking expectations...');
        await expect(error).toMatch(
            'Failed to load avm.wasm. This usually means that the web server is not serving avm file correctly',
        );
    });
});

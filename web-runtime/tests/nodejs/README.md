# Tests for AvmRunnerBackground` in nodejs

Integration tests for `AvmRunnerBackground` in nodejs environment. To simulate full development cycle `avm-runner-background` is installed locally as if it was taken from npm. This is possible thanks to [install-local](https://github.com/nicojs/node-install-local) package. The installation can be simulated by the `npm run install:local` command.

## Running the tests

First build the `avm-runner-background` package.

In `$repo_root/avm-runner-background` run:

```bash
npm i
./build_runner.sh
npm run build
```

Then in `$repo_root/tests/node` run:

```bash
npm install
npm run install:local
npm run test
```

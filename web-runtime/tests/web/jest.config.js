module.exports = {
    preset: 'jest-puppeteer',
    testMatch: ['**/?(*.)+(spec|test).[t]s'],
    testPathIgnorePatterns: ['/node_modules/', 'dist'],
    testMatch: ['**/test/*.spec.ts'],
    transform: {
        '^.+\\.ts?$': 'ts-jest',
    },
};

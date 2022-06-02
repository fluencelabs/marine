module.exports = {
    testEnvironment: 'node',
    testPathIgnorePatterns: ['dist'],
    transform: {
        '^.+\\.(ts|js)x?$': 'ts-jest',
    },
};

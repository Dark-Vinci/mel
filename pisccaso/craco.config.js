const path = require('path');

module.exports = {
    webpack: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
            '@components': path.resolve(__dirname, 'src/components'),
            '@home': path.resolve(__dirname, 'src/HomePage'),
            '@constants': path.resolve(__dirname, 'src/constants'),
        },
    },
};
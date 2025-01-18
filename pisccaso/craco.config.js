const path = require('path');

module.exports = {
    webpack: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
            '@components': path.resolve(__dirname, 'src/components'),
            "@containers": path.resolve(__dirname, 'src/containers'),
            "@pages": path.resolve(__dirname, 'src/pages'),
            "@startup": path.resolve(__dirname, 'src/startup'),
            "@router": path.resolve(__dirname, 'src/router'),
            "@utils": path.resolve(__dirname, 'src/utils'),
            "@hooks": path.resolve(__dirname, 'src/hooks'),
            "@store": path.resolve(__dirname, 'src/store'),
            "@types": path.resolve(__dirname, 'src/types'),
        },
    },
};
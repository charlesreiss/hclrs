const path = require('path');

module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
    },
    experiments: {
        asyncWebAssembly: true,
    },
    mode: 'development',
    module: {
        rules: [
            {
                test: /\.html/i,
                type: 'asset/resource',
            }
        ]
    },
}

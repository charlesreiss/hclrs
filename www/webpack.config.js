const path = require('path');

module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
        publicPath: 'auto',
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
    watch: true,
}

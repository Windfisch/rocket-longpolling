const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
 
module.exports = {
    //context: path.join(__dirname, 'your-app'),
    //context: __dirname,
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                { from: 'static' }
            ]
        })
    ],
    devServer: {
        contentBase: "./dist",
    },
    resolve: {
        alias: {
            'vue$': 'vue/dist/vue.esm.js' // 'vue/dist/vue.common.js' for webpack 1
        }
    }
};

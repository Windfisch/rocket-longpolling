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
};

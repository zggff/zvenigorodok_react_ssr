/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
import path from 'path'
import { CleanWebpackPlugin } from 'clean-webpack-plugin'
import MiniCssExtractPlugin from 'mini-css-extract-plugin'
import { fileURLToPath } from 'url'
import CopyPlugin from 'copy-webpack-plugin'
import webpack from 'webpack'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const buildDirectory = './dist'

export default [
    {
        mode: 'production',
        target: 'web',
        entry: path.resolve(__dirname, './src/ssr.tsx'),

        output: {
            publicPath: '',
            globalObject: 'this',
            path: path.resolve(__dirname, buildDirectory),
            library: 'SSR',
            libraryTarget: 'var',
            filename: 'index.js',
        },
        resolve: {
            extensions: ['.js', '.jsx', '.json', '.ts', '.tsx'],
        },
        module: {
            rules: [
                {
                    test: /\.ts(x?)$/,
                    exclude: path.resolve(__dirname, 'node_modules'),
                    use: [
                        {
                            loader: 'ts-loader',
                        },
                    ],
                },
                {
                    test: /\.(png|jp(e*)g|svg|gif|webp)$/,
                    use: [
                        {
                            loader: 'file-loader',
                            options: {
                                name: './images/[hash]-[name].[ext]',
                            },
                        },
                    ],
                },
                {
                    test: /\.s[ac]ss$/i,
                    use: [
                        MiniCssExtractPlugin.loader,
                        'css-loader',
                        'postcss-loader',
                        'sass-loader',
                    ],
                },
            ],
        },
        plugins: [
            new MiniCssExtractPlugin({
                filename: './styles/[contenthash]-ssr.css',
            }),
            new CopyPlugin({
                patterns: [
                    {
                        from: './public/sitemap.xml',
                    },
                ],
            }),
        ],
    },
    {
        mode: 'production',
        target: 'web',
        entry: path.resolve(__dirname, './src/index.tsx'),
        output: {
            path: path.resolve(__dirname, buildDirectory),
            filename: 'scripts/[contenthash]-bundle.js',
        },
        resolve: {
            extensions: ['.js', '.jsx', '.json', '.ts', '.tsx'],
        },
        module: {
            rules: [
                {
                    test: /\.ts(x?)$/,
                    exclude: path.resolve(__dirname, 'node_modules'),
                    use: [
                        {
                            loader: 'ts-loader',
                        },
                    ],
                },
                {
                    test: /\.(png|jp(e*)g|svg|gif|webp)$/,
                    use: [
                        {
                            loader: 'file-loader',
                            options: {
                                name: './images/[hash]-[name].[ext]',
                            },
                        },
                    ],
                },
                {
                    test: /\.s[ac]ss$/i,
                    use: ['null-loader'],
                },
            ],
        },
        plugins: [
            new MiniCssExtractPlugin({
                filename: './styles/[contenthash]-ssr.css',
            }),
        ],
    },
]

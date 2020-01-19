const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
	entry: "./public/index.js",
	output: { filename: "main.js", path: path.resolve(__dirname, "dist") },
	plugins: [
		new WasmPackPlugin({ crateDirectory: __dirname }),
		new HtmlWebpackPlugin()
	],
	mode: "production",
	devServer: { proxy: { "/api": "http://127.0.0.1:3000" } }
};

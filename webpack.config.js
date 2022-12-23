module.exports = {
    mode: "development",
    entry: "./src/index.js",
    experiments: {
        asyncWebAssembly: true
    },
    output: {
        publicPath: "" // silences error `Uncaught Error: Automatic publicPath is not supported in this browser`
    }
}
module.exports = {
    mode: "production",
    entry: "./src/index.js",
    experiments: {
        asyncWebAssembly: true
    },
    output: {
        publicPath: "" // makes error message `Uncaught Error: Automatic publicPath is not supported in this browser` go away
    }
}
const importObj = {};

// Node
const data = require("fs").readFileSync("target/wasm32-unknown-unknown/release/rust_example.wasm");
WebAssembly.instantiate(data, importObj).then(({instance})=>{
    console.log(instance.exports.my_add_naming(40, 2))
});

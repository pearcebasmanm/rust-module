import moment from "moment";
import { rust } from "rust_module";

let date = new Date();
let fomatted = moment(date).format("LL");
console.log(fomatted);

rust.rustfn();
console.log("Hello from index.js")

// export const jstest = () => console.log("Hello World (Javascript)")

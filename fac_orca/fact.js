const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("target/out.wasm");
  const env = { log: val => console.log(`logged ${val}`), };
  const { instance } = await WebAssembly.instantiate(
    bytes,
    {
      env: {
        log(val) {
          console.log(`log saw ${val}`);
        }
      }
    }
  );
  const result = instance.exports.factorial(5);
  console.log(`factorial(5) = ${result}`);
}
main();
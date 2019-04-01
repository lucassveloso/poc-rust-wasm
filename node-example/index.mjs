import { fibonacci } from "./crate/target/wasm32-unknown-unknown/release/utils.wasm";

function timer (name) {
  const start = new Date();
  return {
    stop: function() {
      const end = new Date();
      const time = end.getTime() - start.getTime();
      console.log(`Timer: ${name} finished in ${time} ms`);
    }
  }
}

function fibonacciJS(n) {
  switch(n) {
    case 0:
      return 1;
    case 1:
      return 1;
    default:
      return fibonacciJS(n - 1) + fibonacciJS(n - 2)
  }
}

const wasm = timer('Fibonacci WASM');
console.log('WASM Result:', fibonacci(40));
wasm.stop();
const js = timer('Fibonacci JS');
console.log('JS Result:', fibonacciJS(40));
js.stop();
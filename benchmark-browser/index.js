function timer (id) {
  const start = new Date();
  return {
    stop: function() {
      const end = new Date();
      const time = end.getTime() - start.getTime();
      document.getElementById(id).innerHTML = `${time} ms`;
    }
  }
}

function filterEvenNumbers(arr) {
  return arr.filter(n => n % 2 === 0);
}

function fibonacciJS(n) {
  if (n <= 1) { return 1; }
  return fibonacciJS(n - 1) + fibonacciJS(n - 2);
}

function runEvenBenchmark(wasmModule) {
  const numbers = 99999;
  const array = Array.apply(null, {length: numbers }).map(Number.call, Number);
  document.getElementById('even-numbers').innerHTML = numbers;

  const wasm = timer('even-wasm-time');
  // wasmModule.filter_even_numbers(array)
  console.log('WASM even filter result:' , wasmModule.filter_even_numbers(array));
  wasm.stop();

  const js = timer('even-js-time');
  // filterEvenNumbers(array)
  console.log('JS even filter result:', filterEvenNumbers(array));
  js.stop();
}

function runfibonacciBenchmark(wasmModule) {
  const numbers = 20;
  document.getElementById('fibonacci-numbers').innerHTML = numbers;

  const wasm = timer('fibonacci-wasm-time');
  // wasmModule.fibonacci(numbers)
  console.log('WASM fibonacci result:' , wasmModule.fibonacci(numbers));
  wasm.stop();
  
  const js = timer('fibonacci-js-time');
  // fibonacciJS(numbers)
  console.log('JS fibonacci result:', fibonacciJS(numbers));
  js.stop();
}

import('./pkg/benchmark_browser').then(wasmModule => {
  runEvenBenchmark(wasmModule);
  runfibonacciBenchmark(wasmModule);
});
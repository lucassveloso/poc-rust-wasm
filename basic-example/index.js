import('./pkg/basic_example').then(wasmModule => {
  wasmModule.greeting('World');
});
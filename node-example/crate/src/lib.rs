#[no_mangle]
pub extern fn fibonacci(n: u32) -> u32 {
  if n <= 1 { return 1 }
  fibonacci(n - 1) + fibonacci(n - 2)
}
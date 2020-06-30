#[cfg(test)]

use fireworks::*;

#[test]
fn fibonacci_default_test() {
  for &(index, value) in [(0, 1), (1, 1), (2, 2), (3, 3), (4, 5), (5, 8), (6, 13)].iter() {
    assert_eq!(fibonacci::fibonacci(index), value);
  }
}
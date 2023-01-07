use quickcheck::{Arbitrary, Gen};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod tests;
pub use tests::*;

/// A test case for `T`, whether it correctly implements a given function.
pub trait Test<T> {
    type Input: Arbitrary + Send + 'static;

    fn eval(input: Self::Input, imp: T) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail,
    Timeout,
}

pub fn run_test<S: Default, T: Test<S>>(timeout: Duration) -> TestResult {
    const NUM_RUNS: usize = 10;
    let (send, rcv) = mpsc::channel();

    let mut g = Gen::new(10);
    for _ in 0..NUM_RUNS {
        let input = T::Input::arbitrary(&mut g);
        let send_clone = send.clone();
        thread::spawn(move || {
            send_clone.send(T::eval(input, S::default())).unwrap();
        });

        match rcv.recv_timeout(timeout) {
            Ok(true) => {}
            Ok(false) => return TestResult::Fail,
            Err(_) => return TestResult::Timeout,
        }
    }

    TestResult::Pass
}

#[macro_export]
macro_rules! generate_test {
  ($test: tt, $imp: tt) => {

    fn main() {
      use surveyor::{TestResult, run_test};
      let dur = std::time::Duration::from_secs(10);
      let exit_code = match run_test::<$imp, $test>(dur) {
        TestResult::Pass => 0,
        TestResult::Fail => 1,
        TestResult::Timeout => 2,
      };
      std::process::exit(exit_code);
    }

  }
}

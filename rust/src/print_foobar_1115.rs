// https://leetcode.com/problems/print-foobar-alternately

use std::sync::{Condvar, Mutex};

pub struct FooBar {
    n: usize,
    turn: Mutex<bool>,
    condition: Condvar,
}

impl FooBar {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            turn: Mutex::new(true),
            condition: Condvar::new(),
        }
    }

    pub fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut turn = self.condition.wait_while(self.turn.lock().unwrap(), |turn| !*turn).unwrap();
            print_foo();
            *turn = false;
            self.condition.notify_one();
        }
    }

    pub fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut turn = self.condition.wait_while(self.turn.lock().unwrap(), |turn| *turn).unwrap();
            print_bar();
            *turn = true;
            self.condition.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::*;
    use std::thread;

    #[test]
    fn official1() {
        // n = 1, output: "foobar"
        let output = Arc::new(Mutex::new(String::new()));
        let foobar = Arc::new(FooBar::new(1));

        let output1 = output.clone();
        let foobar1 = foobar.clone();
        let handle1 = thread::spawn(move || {
            foobar1.foo(|| {
                output1.lock().unwrap().push_str("foo");
            });
        });

        let output2 = output.clone();
        let foobar2 = foobar.clone();
        let handle2 = thread::spawn(move || {
            foobar2.bar(|| {
                output2.lock().unwrap().push_str("bar");
            });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let result = output.lock().unwrap().clone();
        assert!(
            result == "foobar",
            "expected alternating foobar, got: {}",
            result
        );
    }

    #[test]
    fn official2() {
        // n = 2, output: "foobarfoobar"
        let output = Arc::new(Mutex::new(String::new()));
        let foobar = Arc::new(FooBar::new(2));

        let output1 = output.clone();
        let foobar1 = foobar.clone();
        let handle1 = thread::spawn(move || {
                foobar1.foo(|| {
                    output1.lock().unwrap().push_str("foo");
                });
        });
        

        let output2 = output.clone();
        let foobar2 = foobar.clone();
        let handle2 = thread::spawn(move || {
                foobar2.bar(|| {
                    output2.lock().unwrap().push_str("bar");
                });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        let result = output.lock().unwrap().clone();
        // The exact interleaving depends on thread scheduling, but it should
        // contain the right number of each
        assert_eq!("foobarfoobar", result, "expected alternating foobar, got: {}", result);
    }
}

// https://leetcode.com/problems/print-in-order

use std::sync::{Condvar, Mutex};


pub struct Foo {
    step: Mutex<usize>,
    condition: Condvar,
}

impl Foo {
    pub fn new() -> Self {
        Self {
            step: Mutex::new(0),
            condition: Condvar::new(),
        }
    }

    pub fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        print_first();
        let mut step = self.step.lock().unwrap();
        *step = 1;
        self.condition.notify_all();
    }

    pub fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        drop(self.condition.wait_while(self.step.lock().unwrap(), |s| *s < 1).unwrap());
        print_second();
        let mut step = self.step.lock().unwrap();
        *step = 2;
        self.condition.notify_all();
    }

    pub fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        drop(self.condition.wait_while(self.step.lock().unwrap(), |s| *s < 2).unwrap());
        print_third();
        let mut step = self.step.lock().unwrap();
        *step = 3;
        self.condition.notify_all();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn official1() {
        // Input: nums = [1,2,3]
        // Output: "firstsecondthird"
        // Thread order: first -> second -> third (already in order)
        let output = Arc::new(Mutex::new(String::new()));
        let foo = Arc::new(Foo::new());

        let output1 = Arc::clone(&output);
        let foo1 = Arc::clone(&foo);
        let t1 = thread::spawn(move || {
            foo1.first(|| {
                output1.lock().unwrap().push_str("first");
            });
        });

        let output2 = Arc::clone(&output);
        let foo2 = Arc::clone(&foo);
        let t2 = thread::spawn(move || {
            foo2.second(|| {
                output2.lock().unwrap().push_str("second");
            });
        });

        let output3 = Arc::clone(&output);
        let foo3 = Arc::clone(&foo);
        let t3 = thread::spawn(move || {
            foo3.third(|| {
                output3.lock().unwrap().push_str("third");
            });
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        assert_eq!("firstsecondthird", output.lock().unwrap().as_str());
    }

    #[test]
    fn official2() {
        // Input: nums = [1,3,2]
        // Output: "firstsecondthird"
        // Thread order: first -> third -> second (out of order, must be synchronized)
        let output = Arc::new(Mutex::new(String::new()));
        let foo = Arc::new(Foo::new());

        let output1 = Arc::clone(&output);
        let foo1 = Arc::clone(&foo);
        let t1 = thread::spawn(move || {
            foo1.first(|| {
                output1.lock().unwrap().push_str("first");
            });
        });

        let output3 = Arc::clone(&output);
        let foo3 = Arc::clone(&foo);
        let t3 = thread::spawn(move || {
            foo3.third(|| {
                output3.lock().unwrap().push_str("third");
            });
        });

        let output2 = Arc::clone(&output);
        let foo2 = Arc::clone(&foo);
        let t2 = thread::spawn(move || {
            foo2.second(|| {
                output2.lock().unwrap().push_str("second");
            });
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        assert_eq!("firstsecondthird", output.lock().unwrap().as_str());
    }
}

/// [1114] Print in Order
/// Difficulty: Easy
/// Topics: Concurrency
/// Tags: RustMastery
///
/// Suppose we have a class Foo that has three methods: first(), second(), and third(). The same instance of Foo will be passed to three different threads.
/// You must ensure that first() is always executed before second(), and second() is always executed before third().
///
/// Link: https://leetcode.com/problems/print-in-order/

pub struct Foo {
    // Add your internal state here
}

impl Foo {
    pub fn new() -> Self {
        todo!("Initialize the synchronization state")
    }
    
    pub fn first(&self, print_first: impl FnOnce()) {
        todo!("Execute first and signal that it's done")
    }
    
    pub fn second(&self, print_second: impl FnOnce()) {
        todo!("Wait for first to complete, then execute second and signal")
    }
    
    pub fn third(&self, print_third: impl FnOnce()) {
        todo!("Wait for second to complete, then execute third")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_print_in_order() {
        let foo = Arc::new(Foo::new());
        let output = Arc::new(Mutex::new(Vec::new()));

        let t1 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.first(|| out.lock().unwrap().push(1));
            })
        };

        let t2 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.second(|| out.lock().unwrap().push(2));
            })
        };

        let t3 = {
            let foo = Arc::clone(&foo);
            let out = Arc::clone(&output);
            thread::spawn(move || {
                foo.third(|| out.lock().unwrap().push(3));
            })
        };

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        assert_eq!(*output.lock().unwrap(), vec![1, 2, 3]);
    }
}

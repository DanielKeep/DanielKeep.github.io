#![feature(phase)]

#[phase(plugin)] extern crate recurrence;

fn main() {
    let fib = recurrence![ fib[n]: f64 = 0.0, 1.0 ... fib[n-1] + fib[n-2] ];
    for (i,e) in fib.enumerate().take(10) {
        println!("fib[{}] = {}", i, e);
    }
}

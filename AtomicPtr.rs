fn main() {
    use std::sync::atomic::{AtomicPtr, Ordering};

    let some_ptr = AtomicPtr::new(&mut "HELLO");

    let new = &mut "WORLD";
    let mut old = some_ptr.load(Ordering::Relaxed);
    loop {
        match some_ptr.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => break,
            Err(x) => old = x,
        }
    }
    let val = unsafe{*some_ptr.load(Ordering::SeqCst)};
    println!("1. {:?} {:?}", unsafe{*old}, unsafe{*some_ptr.load(Ordering::SeqCst)});
    println!("2. {:?} {:?} {:?}", val, unsafe{*old}, unsafe{*some_ptr.load(Ordering::SeqCst)});


    let ptr = &mut 15;
    let some_ptr  = AtomicPtr::new(ptr);

    let other_ptr   = some_ptr.load(Ordering::SeqCst);
    let another_ptr = &mut 100;

    let value = some_ptr.compare_and_swap(other_ptr, another_ptr, Ordering::Relaxed);
    println!("{:?} {:?}", unsafe{*value}, unsafe{*some_ptr.load(Ordering::SeqCst)})
}

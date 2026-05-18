use std::rc::Rc;

fn main() {
    let a = Rc::new([1,2,3]); 
    println!("Count: {}", Rc::strong_count(&a));
    {
    let b = a.clone();
    println!("Count: {}", Rc::strong_count(&a));
    assert_eq!(a.as_ptr(), b.as_ptr());
    // count gets decremented when dropped out of scope.
    }
    println!("Count: {}", Rc::strong_count(&a));
    // let c = Rc::new([1,4,3]);

}

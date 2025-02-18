fn main() {
    let mut v = vec![1, 2, 3];
    { // use a scope to limit the lifetime
        let ptr = v.as_mut_ptr();
        unsafe {
            *ptr = 10;
        }
    } // v is dropped here
    //To avoid undefined behavior
    let mut v2 = vec![1,2,3];
    let len = v2.len();
    let ptr = v2.as_mut_ptr();
    unsafe{
        for i in 0..len{
            *ptr.add(i) = i + 10;
        }
    }
    println!("Modified Vector: {:?}", v2);
}
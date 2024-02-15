use core::slice;

// this is how you declare static variables (a.k.a. global variables)
// the naming convention is the same as for constants: LIKE_THIS
static mut ANSWER: u32 = 41; // type annotation is required

fn main() {
    /*
     * Unsafe Rust exists for two reasons:
     * 1. static analysis is conservative by nature: Rust will reject a program if it can't guarantee
     * that is memory safe, even though, you, the programmer, know that the program is memory safe
     * 2. the underlying computer hardware is inherently unsafe
     *
     * The unsafe keyword gives you five superpowers:
     * 1. dereference raw pointers
     * 2. call an unsafe function or method
     * 3. access or modify a mutable static variable
     * 4. implement unsafe traits
     * 5. access fields of unions
     * The unsafe keyword doesn't turn off the borrow checker nor does it turn off Rust's safety checks.
     * Unsafe code is not dangerous, but the responsibilty for handling memory appropriately
     * is passed onto the programmer, which can, sometimes, create unexpected behaviour and bugs.
     */

    /*
     * Raw pointers:
     * 1. can bypass Rust borrowing rules (there can be multiple mutable refs or
     * mutable and immutable refs at the same time)
     * 2. it is not guaranteed that they point to valid memory
     * 3. are allowed to be null
     * 4. don't implement any type of automatic cleanup
     *
     * Raw pointers might lead to data races.
     * Raw pointers are useful when interacting with C code or when creating safe abstractions
     * around unsafe code (more on that later).
     */
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    let r3 = &mut num as *mut i32; // mutable raw pointer
    unsafe {
        println!("{}", *r1);
        *r2 = 43;
        println!("{}", *r2);
        *r3 = 95;
        println!("{}", *r3);
        // *r1 = 20; // cannot write to an immutable raw pointer
    }
    let addr = 0x40usize;
    let r4 = addr as *const i32;
    // unsafe {
    //     println!("{}", *r4); // potential segmentation fault
    // }
    // println!("{}", *r1); // it doesn't compile

    // call unsafe functions
    unsafe fn dangerous() {
        // no need to open an unsafe block here as the function has already been declared unsafe
        let num = 84;
        let r1 = &num as *const i32;
        println!("{}", *r1);
    }
    unsafe {
        dangerous();
    }

    // creating safe abstractions around unsafe code
    // safe abstractions can be called anywhere, they don't need to be called inside unsafe blocks
    let mut v = vec![1, 2, 3, 4];
    /*
     * The code below does not compile because I am creating two mutable references on the last instruction!
     * Although we are referencing different parts of the slice, the borrow checker doesn't understand
     * that and raises a compilation error. We know the code is fine, so it's ok to use unsafe code
     * and wrap it inside a safe function
     */
    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();

    //     assert!(mid <= len);

    //     (&mut values[..mid], &mut values[mid..])
    // }
    fn unsafe_inside_safe(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len); // this makes the use of unsafe appropriate
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    println!("{:?}", unsafe_inside_safe(&mut v, 2));
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(2); // this is similar to "unsafe_inside_safe"

    // calling external code
    extern "C" {
        // calling C code
        fn abs(input: i32) -> i32;
    }
    // calling external code is considered unsafe as Rust doesn't know if the external language has its
    // same safety features and guarantees
    unsafe {
        println!("{}", abs(-90));
    }
    // allowing other languages to call functions in Rust
    // the directive below avoids changing the name of the function during the compilation process,
    // so the external language knows the original function name
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("calling Rust from C");
    }

    /*
     * Access and modify mutable static variables: an unsafe block is required as these variables
     * might lead to data races.
     * Static variables and constants are not the same thing.
     * Static variables have a fixed address in memory, constants are allowed to duplicate their data
     * whenever they're used: if you're referencing that constant multiple times in the code,
     * the compiler can replace all the instances of that constant with the concrete value.
     */
    fn add_to_static(count: u32) {
        unsafe {
            ANSWER += count;
        }
    }
    add_to_static(1);
    // i still need an unsafe block here as i am accessing a mutable static variable
    unsafe {
        println!("{}", ANSWER);
    }

    // unsafe traits: a trait is unsafe when at least one of its methods is unsafe
    unsafe trait Foo {}
    unsafe impl Foo for String {}

    // accessing fields of a union:
    // unions are used to interface with unions in C - accessing union fields is unsafe because Rust
    // can’t guarantee the type of the data currently being stored in the union instance
}

use std::vec::Vec;
use std::slice::from_raw_parts;

// Test issue
// https://github.com/maciejhirsz/ordnung/issues/8

#[derive(Debug)]
struct DpDetector(u32);

impl Drop for DpDetector {
    fn drop(&mut self) {
        println!("Drop {}", self.0);
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push(DpDetector(1));
    vec.push(DpDetector(2));

    // we assume that we implement remove function in main function
    // create slice from the raw pointer of vec
    let slice = unsafe { from_raw_parts(vec.as_mut_ptr(), 3) };

    panic!("{:?}", slice);

    println!("{:?}", slice);
}

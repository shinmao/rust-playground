#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cx2/include/cplusobj.h");

        type CPlusObj;

        fn new_CplusObj() -> UniquePtr<CPlusObj>;
    }
}

fn main() {
    let client = ffi::new_CplusObj();
}
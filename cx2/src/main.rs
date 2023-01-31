#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Buf;

        fn next_chunk(buf: &mut Buf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cx2/include/cplusobj.h");

        type CPlusObj;

        fn new_CplusObj();
        fn put(&self, parts: &mut Buf) -> u64;
    }
}

pub struct Buf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

impl Drop for Buf {
    fn drop(&mut self) {
        println!("Buf dropped!");
    }
}

pub fn next_chunk(buf: &mut Buf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    ffi::new_CplusObj();
}
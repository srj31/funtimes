#[no_mangle]
pub extern "C" fn get_string(end: u8) -> *const u8 {
    println!("This was the value put in, {}", end);
    b"Hello C-world\n\0".as_ptr()
}

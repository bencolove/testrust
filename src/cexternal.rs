#[no_mangle]
pub extern "C" fn hello() -> String {
    return String::from("hello Rust");
}
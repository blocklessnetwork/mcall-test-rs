#[link(wasm_import_module = "blockless")]
extern "C" {
    #[link_name = "register"]
    fn register(ptr: u32, len: u32, buf: u32, blen: u32);
}

#[no_mangle]
pub fn method_c() {
    println!("method c");
}

#[no_mangle]
pub fn _initialize() {
    let method = "{\"module\":\"module_c\"}";
    let m_ptr = method.as_ptr();
    unsafe {register(m_ptr as u32, method.len() as u32, 0, 0);}
}

fn main() {}
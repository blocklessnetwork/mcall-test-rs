use std::alloc::Layout;

#[link(wasm_import_module = "blockless")]
extern "C" {
    #[link_name = "register"]
    fn register(ptr: u32, len: u32, buf: u32, blen: u32) -> u32;
}


#[no_mangle]
pub fn method_b(ptr: i32, len: u32, result_ptr: i32, result_len: u32) -> u32 {
    let ptr = ptr as *const u8;
    
    unsafe{
        let result = std::slice::from_raw_parts_mut(result_ptr as *mut u8, result_len as usize);    
        let rs = b"this result from module b";
        let l = std::cmp::min(rs.len(), result.len());
        result[..l].copy_from_slice(&rs[..l]);
        let slice = std::slice::from_raw_parts(ptr, len as usize);
        let params = std::str::from_utf8_unchecked(slice);
        println!("this module b recv from module a: {params}")
    }
    0
}

#[no_mangle]
pub unsafe fn alloc(size: u32) -> i32 {
    let align = std::mem::align_of::<u8>();
    let layout = Layout::from_size_align_unchecked(size as usize, align);
    std::alloc::alloc(layout) as i32
}

#[no_mangle]
pub unsafe fn dealloc(ptr: i32, size: u32) {
    let align = std::mem::align_of::<u8>();
    let layout = Layout::from_size_align_unchecked(size as usize, align);
    std::alloc::dealloc(ptr as *mut u8, layout);
}

#[no_mangle]
pub fn _initialize() {
    let method = "{\"module\":\"module_b\", \"methods\":[\"method_b\"]}";
    let m_ptr = method.as_ptr();
    let mut rs = [0u8; 256];
    unsafe {
        let v = register(m_ptr as u32, method.len() as u32, rs.as_mut_ptr() as u32, rs.len() as u32);
        if v != 0 {
            let str = std::str::from_utf8(&rs).unwrap();
            println!("{str}");
        }
    }
}

fn main() {}
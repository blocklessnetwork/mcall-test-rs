#[link(wasm_import_module = "blockless")]
extern "C" {
    #[link_name = "mcall"]
    fn mcall(ptr: u32, len: u32, buf: u32, blen: u32) -> u32;
}

fn main() {
    unsafe{
        let params = r#"{"mcall":"module_b::method_b", "params": {"test": "values"}}"#;
        let m_ptr = params.as_ptr();
        let mut result = [0u8; 256];
        println!("from module a send the to {}", params);
        let n = mcall(m_ptr as u32, params.len() as u32, result.as_mut_ptr() as u32, result.len() as u32);
        let str = std::str::from_utf8(&result).unwrap();
        if n > 0 {
            println!("error call mcall: {str}")
        } else {
            println!("this recv the result from module b:\n\t{str}")
        }
        
    }
    println!("finish.");
}

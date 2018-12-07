extern crate second;

#[no_mangle]
pub extern "C" fn add(a: i32, b:i32) -> i32 {
    eprint!("\tRust: first::add({}, {}) -> ", a, b);
    a + b
}

// Uses second as a dynamic Rust dependency
#[no_mangle]
pub extern "C" fn mult(a: i32, b:i32) -> i32 {
    eprint!("\tRust: first::mult({}, {}) -> ", a, b);
    second::mult_exported(a, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(4, 6), 10);
        assert_eq!(mult(4, 6), 24);
    }
}

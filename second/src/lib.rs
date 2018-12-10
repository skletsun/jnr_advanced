#[no_mangle]
pub extern "C" fn sub(a: i32, b:i32) -> i32 {
    eprint!("\tRust: second::sub({}, {}) -> ", a, b);
    a - b
}

// Dynamically called by `first`
#[no_mangle]
pub fn mult_exported(a: i32, b:i32) -> i32 {
    eprint!("\tRust: second::mult_exported({}, {}) -> ", a, b);
    a * b
}

#[no_mangle]
pub fn div_native(a: i32, b:i32) -> i32 {
    eprint!("\tRust: second::div_native({}, {}) -> ", a, b);
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sub(2, 1), 1);
        assert_eq!(sub(2, 3), -1);
        assert_eq!(sub(1, 1), 0);
        assert_eq!(mult_exported(2, 3), 6);
        assert_eq!(div_native(10, 5), 2);
    }
}

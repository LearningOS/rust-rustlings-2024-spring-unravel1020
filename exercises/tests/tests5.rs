// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///  
/// The `address` must contain a mutable reference to a valid `u32` value.  
unsafe fn modify_by_address(address: usize) {  
    // SAFETY: We assume that the caller has provided a valid mutable address  
    // pointing to a `u32` value. It is the caller's responsibility to ensure  
    // that the address is valid and that it points to a memory location that  
    // can be safely mutated.  
    unsafe {  
        // Convert the usize address back to a raw mutable pointer.  
        let ptr: *mut u32 = address as *mut u32;  
        // Write the new value to the memory location pointed by the pointer.  
        *ptr = 0xAABBCCDD;  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn test_success() {  
        let mut t: u32 = 0x12345678;  
        // SAFETY: The address is guaranteed to be valid and contains  
        // a unique reference to a `u32` local variable.  
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };  
        assert_eq!(t, 0xAABBCCDD);  
    }  
}
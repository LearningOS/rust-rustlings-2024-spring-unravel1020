// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {  
    fn version(&self) -> String;  
      
    // 提供 licensing_info 的默认实现  
    fn licensing_info(&self) -> String {  
        format!("Licensed under version: {}", self.version())  
    }  
}  
  
struct SomeSoftware {  
    version_number: i32,  
}  
  
struct OtherSoftware {  
    version_number: String,  
}  
  
impl Licensed for SomeSoftware {  
    fn version(&self) -> String {  
        format!("{}", self.version_number)  
    }  
}  
  
impl Licensed for OtherSoftware {  
    fn version(&self) -> String {  
        self.version_number.clone()  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn is_licensing_info_the_same() {  
        let licensing_info = "Licensed under version: 1".to_string();  
        let some_software = SomeSoftware { version_number: 1 };  
        assert_eq!(some_software.licensing_info(), licensing_info);  
  
        let licensing_info_other = "Licensed under version: v2.0.0".to_string();  
        let other_software = OtherSoftware { version_number: "v2.0.0".to_string() };  
        assert_eq!(other_software.licensing_info(), licensing_info_other);  
    }  
}
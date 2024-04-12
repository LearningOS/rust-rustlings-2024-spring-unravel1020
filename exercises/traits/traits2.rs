// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {  
    fn append_bar(self) -> Self;  
}  
  
impl AppendBar for Vec<String> {  
    fn append_bar(self) -> Self {  
        // 创建一个新的字符串"Bar"  
        let bar_string = String::from("Bar");  
        // 在向量的末尾添加"Bar"字符串，并返回新的向量  
        let mut new_vec = self;  
        new_vec.push(bar_string);  
        new_vec  
    }  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn is_vec_pop_eq_bar() {  
        // 创建一个只包含"Foo"的字符串向量，并调用append_bar方法  
        let mut foo = vec![String::from("Foo")].append_bar();  
        // 断言最后一个元素是"Bar"  
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));  
        // 断言倒数第二个元素是"Foo"  
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));  
        // 断言向量现在是空的  
        assert!(foo.is_empty());  
    }  
}
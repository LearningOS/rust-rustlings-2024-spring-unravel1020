// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


trait AppendBar {  
    fn append_bar(self) -> Self;  
}  
  
impl AppendBar for String {  
    fn append_bar(self) -> Self {  
        let mut new_string = self.clone(); // 克隆原始字符串以获取其副本的所有权  
        new_string.push_str("Bar"); // 在副本上追加"Bar"  
        new_string // 返回修改后的字符串  
    }  
}  
  
fn main() {  
    let s = String::from("Foo");  
    let s_bar = s.append_bar(); // 注意这里我们需要接收返回的新字符串  
    println!("s: {}", s_bar); // 打印出修改后的字符串  
}  
  
#[cfg(test)]  
mod tests {  
    use super::*;  
  
    #[test]  
    fn is_foo_bar() {  
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));  
    }  
  
    #[test]  
    fn is_bar_bar() {  
        // 由于`append_bar`消耗了字符串，我们需要再次调用`.clone()`来保留一个用于第二次调用的副本  
        assert_eq!(  
            String::from("")  
                .append_bar()  
                .clone() // 克隆第一次调用`append_bar`的结果  
                .append_bar(), // 在克隆的字符串上再次调用`append_bar`  
            String::from("BarBar")  
        );  
    }  
}
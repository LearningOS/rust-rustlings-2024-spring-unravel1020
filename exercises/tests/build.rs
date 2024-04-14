//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {  
    // 获取当前UNIX时间戳  
    let timestamp = std::time::SystemTime::now()  
        .duration_since(std::time::UNIX_EPOCH)  
        .unwrap()  
        .as_secs();  
      
    // 设置环境变量TEST_FOO为当前UNIX时间戳  
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);  
    println!("cargo:{}", your_command);  
  
    // 下面的行是为tests8准备的，如果你还没完成tests8，可以先注释掉  
    println!("cargo:rustc-cfg=feature=\"pass\"");  
}
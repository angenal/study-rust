mod study; // 引入模块, 在对应目录中实现模块功能
use std::{ffi::CString, path::Path};

fn main() {
    let path = "D:/Temp/WebView2";
    if !Path::new(&path).exists() {
        print!("删除目录：不存在 -> {}", path);
        return;
    }
    let data = CString::new(path).unwrap();
    let ok = study::file::rs_rm_dir(data.as_ptr());
    print!("删除目录：{} -> {}", ok, path);

    // study::fmt::run();

    // study::input::run();
}

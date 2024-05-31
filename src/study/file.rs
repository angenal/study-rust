use std::{ffi::CStr, fs, os::raw::c_char, path::Path};

#[no_mangle]
pub extern "C" fn rs_rm_dir(path: *const c_char) -> bool {
    let c_str = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path)
    };

    // c 字符串 转为 rust 字符串
    let path_str = c_str.to_str().unwrap().to_string();

    println!("地址:");
    println!("{}", path_str);

    let ret = {
        if Path::new(&path_str).exists() {
            match fs::remove_dir_all(&path_str) {
                Ok(_) => true,
                Err(err) => {
                    println!("{}", err);
                    return false;
                }
            }
        } else {
            true
        }
    };

    if ret {
        // 检查一下是否真的删掉了
        return !Path::new(&path_str).exists();
    }

    return false;
}

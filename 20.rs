// TODO: Удалите эту строчку, когда все будет готово.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // См. неопределенные типы (opaque) https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Раскладка согласно ман странице Linux для функции readdir(3), где ino_t и
    // off_t соответствуют определениям в
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Раскладка в соответствии в ман страницей macOS для dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    unsafe extern "C" {
        pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        // См. https://github.com/rust-lang/libc/issues/414 и раздел
        // _DARWIN_FEATURE_64_BIT_INODE в ман страницах macOS для stat(2).
        //
        // "Platforms that existed before these updates were available" это ссылка на
        // macOS (в противоположность iOS / wearOS / и пр.) на Intel и PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

        pub unsafe fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // вызовет ошибку, если в конце не \0 илив середине есть \0
        let cstr = CString::new(path).map_err(|e| format!("Невалидный путь: {}", e))?;
        let dir = unsafe{ffi::opendir(cstr.as_ptr())};
        if dir.is_null(){
            Err(format!("Нулевой указатель. Директория: {}", path))
        } else {
            Ok(DirectoryIterator{path: cstr, dir: dir})
        }
        // Вызовите opendir и верните значение Ok если она сработала,
        // иначе Err с сообщением.
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        let a = unsafe{ffi::readdir(self.dir)};
        if a.is_null(){
            return None;
        } else {
            // преобразуем сырой указатель *const ffi::dirent в безопасную ссылку rust (читаем значение по ссылке и создаём на него ссылку)
            let b = unsafe{&*a};
            // указатель на массив типа c_char (*const c_char)
            let c = b.d_name.as_ptr();
            let cstr = unsafe{CStr::from_ptr(c)};
            let bytes = cstr.to_bytes();
            let osstr = OsStr::from_bytes(bytes);
            
            return Some(osstr.to_os_string());
        }
        // KПродолжайте вызыват readdir пока не получите нулевой узазатель.
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Вызовите как надо closedir.
        unsafe{
            ffi::closedir(self.dir);
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("файлы: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

use std::{
    ffi::{CStr, CString, c_char},
    fmt::Display,
};

#[derive(Debug)]
pub struct StringFfi {
    c_string: CString,
}

impl StringFfi {
    pub fn from_c_str(s: &CStr) -> Self {
        Self {
            c_string: CString::new(s.to_bytes()).unwrap(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        Self {
            c_string: CString::new(s).expect("CString::new failed"),
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.c_string.as_ptr()
    }

    pub fn from_i8_array(arr: &[i8]) -> Self {
        let len = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
        let slice = &arr[..len];
        let bytes: Vec<u8> = slice.iter().map(|&c| c as u8).collect();
        let string = String::from_utf8_lossy(&bytes).to_string();
        let c_string = CString::new(string.clone()).unwrap();
        Self { c_string }
    }
}

impl Display for StringFfi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c_string.to_string_lossy())
    }
}

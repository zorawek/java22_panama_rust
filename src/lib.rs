use std::ffi::{CStr, CString};

use pdf_extract::extract_text;

#[no_mangle]
pub extern "C" fn extract_text_from_pdf(pdf_path: *const i8, search_for: *const i8) -> bool {
    unsafe {
        let path_str = CStr::from_ptr(pdf_path).to_str().unwrap();
        let search_for_str = CStr::from_ptr(search_for).to_str().unwrap();
        match extract_text(&path_str) {
            Ok(text) => {
                let text_cstring = CString::new(text).unwrap();
                text_cstring.to_str().unwrap().contains(search_for_str)
            }
            Err(_) => {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}

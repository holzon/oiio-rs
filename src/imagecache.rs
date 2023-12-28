use crate::ffi;
use std::ffi::{CStr, CString};

pub struct ImageCache {
    pub cache: ffi::ImageCache,
}

impl Drop for ImageCache {
    fn drop(&mut self) {
        unsafe { ffi::ImageCache_destroy(self.cache) };
    }
}

impl ImageCache {
    pub fn new() -> ImageCache {
        ImageCache {
            cache : unsafe {
                ffi::ImageCache_create()
            }
        }   
    }

    pub fn set_int_attribute(&self, name: &str, value: i32) -> bool {
        let name = CString::new(name).unwrap();
        unsafe {
            ffi::ImageCache_set_int_attribute(self.cache, name.as_ptr(), value)
        }
    }

    pub fn set_float_attribute(&self, name: &str, value: f32) -> bool {
        let name = CString::new(name).unwrap();
        unsafe {
            ffi::ImageCache_set_float_attribute(self.cache, name.as_ptr(), value)
        }
    }

    pub fn set_string_attribute(&self, name: &str, value: &str) -> bool {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe {
            ffi::ImageCache_set_string_attribute(self.cache, name.as_ptr(), value.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_image_cache() {
        let _image_cache = ImageCache::new();
    }

    #[test]
    fn set_image_cache_attribute() {
        let image_cache = ImageCache::new();
        assert_eq!(true, image_cache.set_int_attribute("forcefloat", 1));
    }

    #[test]
    fn set_image_cache_attribute_invalid() {
        let image_cache = ImageCache::new();
        assert_eq!(false, image_cache.set_int_attribute("forc_efloat", 1));
    }
}
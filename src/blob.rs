use cxx::UniquePtr;
use zim_sys::binding::ffi;

use crate::cxx::string_from_char_array;

pub struct Blob {
    ptr: UniquePtr<ffi::Blob>,
}

impl Blob {
    pub(crate) fn from_ptr(ptr: UniquePtr<ffi::Blob>) -> Result<Blob, ()> {
        match ptr.is_null() {
            true => Err(()),
            false => Ok(Blob { ptr }),
        }
    }

    fn inner_ref(&self) -> &ffi::Blob {
        self.ptr.as_ref().unwrap()
    }

    pub fn data(&self) -> Result<String, ()> {
        unsafe { string_from_char_array(ffi::blob_data(self.inner_ref())) }
    }

    pub fn size(&self) -> u64 {
        ffi::blob_size(self.inner_ref())
    }
}

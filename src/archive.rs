use cxx::UniquePtr;
use zim_sys::binding::ffi;

use crate::{
    cxx::{string_from_ptr, vec_string_from_ptr},
    entry::Entry,
    item::Item,
    uuid::Uuid,
};

pub struct Archive {
    ptr: UniquePtr<ffi::Archive>,
}

impl Archive {
    fn inner_ref(&self) -> &ffi::Archive {
        self.ptr.as_ref().unwrap()
    }

    pub fn new(path: &str) -> Result<Archive, ()> {
        let ptr = ffi::archive_ctor_file(path);
        match ptr.is_null() {
            true => Err(()),
            false => Ok(Archive { ptr }),
        }
    }

    pub fn get_filename(&self) -> String {
        ffi::archive_getFilename(self.inner_ref()).to_string()
    }

    pub fn get_filesize(&self) -> u64 {
        ffi::archive_getFilesize(self.inner_ref())
    }

    pub fn get_all_entrycount(&self) -> u32 {
        ffi::archive_getAllEntryCount(self.inner_ref())
    }

    pub fn get_entrycount(&self) -> u32 {
        ffi::archive_getEntryCount(self.inner_ref())
    }

    pub fn get_articlecount(&self) -> u32 {
        ffi::archive_getArticleCount(self.inner_ref())
    }

    pub fn get_uuid(&self) -> Result<Uuid, ()> {
        Uuid::from_ptr(ffi::archive_getUuid(self.inner_ref()))
    }

    pub fn get_metadata(&self, name: &str) -> Result<String, ()> {
        let metadata = ffi::archive_getMetadata(self.inner_ref(), name);
        string_from_ptr(metadata)
    }

    pub fn get_metadata_item(&self, name: &str) -> Result<Item, ()> {
        Item::from_ptr(ffi::archive_getMetadataItem(self.inner_ref(), name))
    }

    pub fn get_metadata_keys(&self) -> Result<Vec<String>, ()> {
        let keys = ffi::archive_getMetadataKeys(self.inner_ref());
        vec_string_from_ptr(keys)
    }

    pub fn get_entry_bypath_index(&self, index: u32) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getEntryByPath_idx(self.inner_ref(), index))
    }

    pub fn get_entry_bypath_str(&self, path: &str) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getEntryByPath_str(self.inner_ref(), path))
    }

    pub fn get_entry_bytitle_index(&self, index: u32) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getEntryByTitle_idx(self.inner_ref(), index))
    }

    pub fn get_entry_bytitle_str(&self, title: &str) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getEntryByTitle_str(self.inner_ref(), title))
    }

    pub fn get_mainentry(&self) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getMainEntry(self.inner_ref()))
    }

    pub fn get_randomentry(&self) -> Result<Entry, ()> {
        Entry::from_ptr(ffi::archive_getRandomEntry(self.inner_ref()))
    }

    pub fn has_entry_bypath(&self, path: &str) -> bool {
        ffi::archive_hasEntryByPath(self.inner_ref(), path)
    }

    pub fn has_entry_bytitle(&self, title: &str) -> bool {
        ffi::archive_hasEntryByTitle(self.inner_ref(), title)
    }

    pub fn has_mainentry(&self) -> bool {
        ffi::archive_hasMainEntry(self.inner_ref())
    }

    pub fn has_fulltext_index(&self) -> bool {
        ffi::archive_hasFulltextIndex(self.inner_ref())
    }

    pub fn has_checksum(&self) -> bool {
        ffi::archive_hasChecksum(self.inner_ref())
    }

    pub fn get_checksum(&self) -> Result<String, ()> {
        let checksum = ffi::archive_getChecksum(self.inner_ref());
        string_from_ptr(checksum)
    }

    pub fn check(&self) -> bool {
        ffi::archive_check(self.inner_ref())
    }

    pub fn is_multipart(&self) -> bool {
        ffi::archive_isMultiPart(self.inner_ref())
    }

    pub fn has_new_namespace_scheme(&self) -> bool {
        ffi::archive_hasNewNamespaceScheme(self.inner_ref())
    }
}

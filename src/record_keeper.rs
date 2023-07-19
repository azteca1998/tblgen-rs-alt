use std::ffi::CString;

use crate::{record::Record, record_map::RecordMap};
use tablegen_sys::{
    tableGenRecordKeeperGetAllDerivedDefinitions, tableGenRecordKeeperGetClass,
    tableGenRecordKeeperGetClasses, tableGenRecordKeeperGetDef, tableGenRecordKeeperGetDefs,
    tableGenRecordVectorFree, tableGenRecordVectorGet, TableGenRecordKeeperRef,
    TableGenRecordVectorRef,
};

pub struct RecordKeeper {
    raw: TableGenRecordKeeperRef,
}

impl RecordKeeper {
    pub unsafe fn from_raw(ptr: TableGenRecordKeeperRef) -> RecordKeeper {
        RecordKeeper { raw: ptr }
    }

    pub fn classes(&self) -> RecordMap {
        RecordMap::from_raw(unsafe { tableGenRecordKeeperGetClasses(self.raw) })
    }

    pub fn defs(&self) -> RecordMap {
        RecordMap::from_raw(unsafe { tableGenRecordKeeperGetDefs(self.raw) })
    }

    pub fn get_class(&self, name: &str) -> Option<Record> {
        unsafe {
            let name = CString::new(name).ok()?;
            let class = tableGenRecordKeeperGetClass(self.raw, name.as_ptr());
            if class.is_null() {
                None
            } else {
                Some(Record::from_raw(class))
            }
        }
    }

    pub fn get_def(&self, name: &str) -> Option<Record> {
        unsafe {
            let name = CString::new(name).ok()?;
            let def = tableGenRecordKeeperGetDef(self.raw, name.as_ptr());
            if def.is_null() {
                None
            } else {
                Some(Record::from_raw(def))
            }
        }
    }

    pub fn get_all_derived_definitions(&self, name: &str) -> RecordIterator {
        let name = CString::new(name).unwrap();
        unsafe {
            RecordIterator::from_raw_vector(tableGenRecordKeeperGetAllDerivedDefinitions(
                self.raw,
                name.as_ptr(),
            ))
        }
    }
}

pub struct RecordIterator {
    raw: TableGenRecordVectorRef,
    index: usize,
}

impl RecordIterator {
    unsafe fn from_raw_vector(ptr: TableGenRecordVectorRef) -> RecordIterator {
        RecordIterator { raw: ptr, index: 0 }
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Record> {
        let next = unsafe { tableGenRecordVectorGet(self.raw, self.index) };
        self.index += 1;
        if next.is_null() {
            None
        } else {
            unsafe { Some(Record::from_raw(next)) }
        }
    }
}

impl Drop for RecordIterator {
    fn drop(&mut self) {
        unsafe { tableGenRecordVectorFree(self.raw) }
    }
}

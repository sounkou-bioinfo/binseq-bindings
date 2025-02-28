use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::path::Path;
use std::ptr;
use std::sync::Mutex;

use binseq::{MmapReader, RefRecord};

// This will setup a global error message
static LAST_ERROR: Mutex<Option<CString>> = Mutex::new(None);

// Set last error message
fn set_last_error(msg: &str) {
    let c_string = CString::new(msg)
        .unwrap_or_else(|_| CString::new("Error message contains invalid UTF-8").unwrap());

    let mut error = LAST_ERROR.lock().unwrap();
    *error = Some(c_string);
}

// Opaque types for C interface
/// Opaque reader type
pub struct BinseqReader {
    reader: MmapReader,
}

/// Opaque record type with 'static lifetime
/// The C user must ensure the reader outlives any records
pub struct BinseqRecord {
    // Use a raw pointer to avoid lifetime issues
    record: *mut c_void,
}

impl BinseqRecord {
    fn new(record: RefRecord<'static>) -> Self {
        let boxed = Box::new(record);
        let ptr = Box::into_raw(boxed) as *mut c_void;
        Self { record: ptr }
    }

    fn as_ref(&self) -> &RefRecord<'static> {
        unsafe { &*(self.record as *const RefRecord) }
    }
}

impl Drop for BinseqRecord {
    fn drop(&mut self) {
        if !self.record.is_null() {
            unsafe {
                drop(Box::from_raw(self.record as *mut RefRecord));
            }
            self.record = ptr::null_mut();
        }
    }
}

/// Opens a binseq file and returns a reader
///
/// # Safety
/// The caller must free the reader with binseq_reader_close when done
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_open(path: *const c_char) -> *mut BinseqReader {
    if path.is_null() {
        set_last_error("Null path provided");
        return ptr::null_mut();
    }

    let c_path = CStr::from_ptr(path);
    let path_str = match c_path.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("Invalid UTF-8 in path");
            return ptr::null_mut();
        }
    };

    match MmapReader::new(Path::new(path_str)) {
        Ok(reader) => {
            let binseq_reader = BinseqReader { reader };
            Box::into_raw(Box::new(binseq_reader))
        }
        Err(err) => {
            set_last_error(&format!("Failed to open file: {}", err));
            ptr::null_mut()
        }
    }
}

/// Closes and frees a binseq reader
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_close(reader: *mut BinseqReader) {
    if !reader.is_null() {
        drop(Box::from_raw(reader));
    }
}

/// Returns the number of records in the file
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_num_records(reader: *const BinseqReader) -> usize {
    if reader.is_null() {
        return 0;
    }
    (*reader).reader.num_records()
}

/// Returns the primary sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_slen(reader: *const BinseqReader) -> u32 {
    if reader.is_null() {
        return 0;
    }
    (*reader).reader.header().slen
}

/// Returns the paired sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_xlen(reader: *const BinseqReader) -> u32 {
    if reader.is_null() {
        return 0;
    }
    (*reader).reader.header().xlen
}

/// Gets a record by index
///
/// # Safety
/// The caller must free the record with binseq_record_free when done
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_get_record(
    reader: *const BinseqReader,
    idx: usize,
) -> *mut BinseqRecord {
    if reader.is_null() {
        set_last_error("Null reader");
        return ptr::null_mut();
    }

    let record_result = (*reader).reader.get(idx);

    match record_result {
        Ok(record) => {
            // Transmute the lifetime to 'static - this is unsafe but necessary for C FFI
            let record: RefRecord<'static> = std::mem::transmute(record);
            let binseq_record = BinseqRecord::new(record);
            Box::into_raw(Box::new(binseq_record))
        }
        Err(err) => {
            set_last_error(&format!("Failed to get record: {}", err));
            ptr::null_mut()
        }
    }
}

/// Frees a record
#[no_mangle]
pub unsafe extern "C" fn binseq_record_free(record: *mut BinseqRecord) {
    if !record.is_null() {
        drop(Box::from_raw(record));
    }
}

/// Gets the flag for a record
#[no_mangle]
pub unsafe extern "C" fn binseq_record_flag(record: *const BinseqRecord) -> u64 {
    if record.is_null() {
        return 0;
    }
    (*record).as_ref().flag()
}

/// Checks if a record is paired
#[no_mangle]
pub unsafe extern "C" fn binseq_record_is_paired(record: *const BinseqRecord) -> bool {
    if record.is_null() {
        return false;
    }
    (*record).as_ref().paired()
}

/// Decodes the primary sequence into a buffer
/// Returns the actual sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_record_decode_primary(
    record: *const BinseqRecord,
    buffer: *mut c_char,
    buffer_size: usize,
) -> usize {
    if record.is_null() || buffer.is_null() {
        return 0;
    }

    let mut dbuf = Vec::new();
    let result = (*record).as_ref().decode_s(&mut dbuf);

    match result {
        Ok(_) => {
            let copy_size = std::cmp::min(dbuf.len(), buffer_size);
            std::ptr::copy_nonoverlapping(dbuf.as_ptr() as *const c_char, buffer, copy_size);
            dbuf.len()
        }
        Err(err) => {
            set_last_error(&format!("Failed to decode sequence: {}", err));
            0
        }
    }
}

/// Decodes the paired sequence into a buffer
/// Returns the actual sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_record_decode_extended(
    record: *const BinseqRecord,
    buffer: *mut c_char,
    buffer_size: usize,
) -> usize {
    if record.is_null() || buffer.is_null() {
        return 0;
    }

    let mut dbuf = Vec::new();
    let result = (*record).as_ref().decode_x(&mut dbuf);

    match result {
        Ok(_) => {
            let copy_size = std::cmp::min(dbuf.len(), buffer_size);
            std::ptr::copy_nonoverlapping(dbuf.as_ptr() as *const c_char, buffer, copy_size);
            dbuf.len()
        }
        Err(err) => {
            set_last_error(&format!("Failed to decode paired sequence: {}", err));
            0
        }
    }
}

/// Gets the last error message
#[no_mangle]
pub extern "C" fn binseq_last_error() -> *const c_char {
    let error = LAST_ERROR.lock().unwrap();
    error.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())
}

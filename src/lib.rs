use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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
/// This version allows records to be reused
pub struct BinseqRecord {
    // This is either None (empty record) or Some(record)
    record: Option<RefRecord<'static>>,
}

/// Opaque buffer context to avoid reallocations
pub struct BinseqContext {
    // Reusable buffer for primary sequence decoding
    sbuf: Vec<u8>,
    // Reusable buffer for extended sequence decoding
    xbuf: Vec<u8>,
}

impl BinseqRecord {
    fn new() -> Self {
        Self { record: None }
    }

    fn set(&mut self, record: RefRecord<'static>) {
        self.record = Some(record);
    }

    fn as_ref(&self) -> Option<&RefRecord<'static>> {
        self.record.as_ref()
    }

    fn is_empty(&self) -> bool {
        self.record.is_none()
    }

    fn clear(&mut self) {
        self.record = None;
    }
}

/// Creates a new empty record for reuse
///
/// # Safety
/// The caller must free the record with binseq_record_free when done
#[no_mangle]
pub extern "C" fn binseq_record_new() -> *mut BinseqRecord {
    let record = BinseqRecord::new();
    Box::into_raw(Box::new(record))
}

/// Creates a new decoding context with pre-allocated buffers
///
/// # Safety
/// The caller must free the context with binseq_context_free when done
#[no_mangle]
pub extern "C" fn binseq_context_new() -> *mut BinseqContext {
    let context = BinseqContext {
        sbuf: Vec::default(), // Initial capacity, will grow as needed
        xbuf: Vec::default(), // Initial capacity, will grow as needed
    };
    Box::into_raw(Box::new(context))
}

/// Frees a context
#[no_mangle]
pub unsafe extern "C" fn binseq_context_free(context: *mut BinseqContext) {
    if !context.is_null() {
        drop(Box::from_raw(context));
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

/// Gets a record by index and stores it in a reusable record container
/// Returns true if successful, false on error
///
/// # Safety
/// The record container must be created with binseq_record_new
#[no_mangle]
pub unsafe extern "C" fn binseq_reader_get_record(
    reader: *const BinseqReader,
    idx: usize,
    record_container: *mut BinseqRecord,
) -> bool {
    if reader.is_null() {
        set_last_error("Null reader");
        return false;
    }

    if record_container.is_null() {
        set_last_error("Null record container");
        return false;
    }

    let record_result = (*reader).reader.get(idx);

    match record_result {
        Ok(record) => {
            // Transmute the lifetime to 'static - this is unsafe but necessary for C FFI
            let record: RefRecord<'static> = std::mem::transmute(record);
            // Store the record in the container
            (*record_container).set(record);
            true
        }
        Err(err) => {
            set_last_error(&format!("Failed to get record: {}", err));
            // Clear the record container
            (*record_container).clear();
            false
        }
    }
}

/// Clear a record container (does not free it)
#[no_mangle]
pub unsafe extern "C" fn binseq_record_clear(record: *mut BinseqRecord) {
    if !record.is_null() {
        (*record).clear();
    }
}

/// Check if a record container is empty (contains no valid record)
#[no_mangle]
pub unsafe extern "C" fn binseq_record_is_empty(record: *const BinseqRecord) -> bool {
    if record.is_null() {
        return true;
    }
    (*record).is_empty()
}

/// Frees a record container
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
    match (*record).as_ref() {
        Some(r) => r.flag(),
        None => 0,
    }
}

/// Checks if a record is paired
#[no_mangle]
pub unsafe extern "C" fn binseq_record_is_paired(record: *const BinseqRecord) -> bool {
    if record.is_null() {
        return false;
    }
    match (*record).as_ref() {
        Some(r) => r.paired(),
        None => false,
    }
}

/// Decodes the primary sequence into the context's internal buffer
/// Returns the sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_record_decode_primary(
    record: *const BinseqRecord,
    context: *mut BinseqContext,
) -> usize {
    if record.is_null() || context.is_null() {
        return 0;
    }

    // Get the record
    let r = match (*record).as_ref() {
        Some(r) => r,
        None => {
            set_last_error("Empty record");
            return 0;
        }
    };

    // Use the reusable buffer from the context
    (*context).sbuf.clear();
    let result = r.decode_s(&mut (*context).sbuf);

    match result {
        Ok(_) => (*context).sbuf.len(),
        Err(err) => {
            set_last_error(&format!("Failed to decode sequence: {}", err));
            0
        }
    }
}

/// Decodes the extended sequence into the context's internal buffer
/// Returns the sequence length
#[no_mangle]
pub unsafe extern "C" fn binseq_record_decode_extended(
    record: *const BinseqRecord,
    context: *mut BinseqContext,
) -> usize {
    if record.is_null() || context.is_null() {
        return 0;
    }

    // Get the record
    let r = match (*record).as_ref() {
        Some(r) => r,
        None => {
            set_last_error("Empty record");
            return 0;
        }
    };

    // Use the reusable buffer from the context
    (*context).xbuf.clear();
    let result = r.decode_x(&mut (*context).xbuf);

    match result {
        Ok(_) => (*context).xbuf.len(),
        Err(err) => {
            set_last_error(&format!("Failed to decode paired sequence: {}", err));
            0
        }
    }
}

/// Gets a pointer to the primary sequence buffer in the context
/// The pointer is only valid until the next call to any decode function
#[no_mangle]
pub unsafe extern "C" fn binseq_context_primary_ptr(
    context: *const BinseqContext,
) -> *const c_char {
    if context.is_null() || (*context).sbuf.is_empty() {
        return ptr::null();
    }
    (*context).sbuf.as_ptr() as *const c_char
}

/// Gets the length of the primary sequence buffer in the context
#[no_mangle]
pub unsafe extern "C" fn binseq_context_primary_len(context: *const BinseqContext) -> usize {
    if context.is_null() {
        return 0;
    }
    (*context).sbuf.len()
}

/// Gets a pointer to the extended sequence buffer in the context
/// The pointer is only valid until the next call to any decode function
#[no_mangle]
pub unsafe extern "C" fn binseq_context_extended_ptr(
    context: *const BinseqContext,
) -> *const c_char {
    if context.is_null() || (*context).xbuf.is_empty() {
        return ptr::null();
    }
    (*context).xbuf.as_ptr() as *const c_char
}

/// Gets the length of the extended sequence buffer in the context
#[no_mangle]
pub unsafe extern "C" fn binseq_context_extended_len(context: *const BinseqContext) -> usize {
    if context.is_null() {
        return 0;
    }
    (*context).xbuf.len()
}

/// Copies the primary sequence to a user-provided buffer
/// Returns the number of bytes copied
#[no_mangle]
pub unsafe extern "C" fn binseq_context_copy_primary(
    context: *const BinseqContext,
    buffer: *mut c_char,
    buffer_size: usize,
) -> usize {
    if context.is_null() || buffer.is_null() || (*context).sbuf.is_empty() {
        return 0;
    }

    let copy_size = std::cmp::min((*context).sbuf.len(), buffer_size);
    std::ptr::copy_nonoverlapping((*context).sbuf.as_ptr() as *const c_char, buffer, copy_size);
    copy_size
}

/// Copies the extended sequence to a user-provided buffer
/// Returns the number of bytes copied
#[no_mangle]
pub unsafe extern "C" fn binseq_context_copy_extended(
    context: *const BinseqContext,
    buffer: *mut c_char,
    buffer_size: usize,
) -> usize {
    if context.is_null() || buffer.is_null() || (*context).xbuf.is_empty() {
        return 0;
    }

    let copy_size = std::cmp::min((*context).xbuf.len(), buffer_size);
    std::ptr::copy_nonoverlapping((*context).xbuf.as_ptr() as *const c_char, buffer, copy_size);
    copy_size
}

/// Gets the last error message
#[no_mangle]
pub extern "C" fn binseq_last_error() -> *const c_char {
    let error = LAST_ERROR.lock().unwrap();
    error.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())
}

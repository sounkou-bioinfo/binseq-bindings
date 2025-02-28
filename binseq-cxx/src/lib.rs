use binseq::{MmapReader, RefRecord as BinseqRefRecord};
use std::path::Path;

// Create wrapper types for the binseq library
pub struct BinseqReaderWrapper {
    reader: MmapReader,
}

pub struct RecordWrapper<'a> {
    record: BinseqRefRecord<'a>,
}

// The bridge between Rust and C++
#[cxx::bridge]
mod ffi {

    // Opaque Rust types exposed to C++
    #[allow(clippy::needless_lifetimes)]
    extern "Rust" {
        type BinseqReaderWrapper;
        type RecordWrapper<'a>;

        // Reader methods
        fn open_mmap_reader(path: &str) -> Result<Box<BinseqReaderWrapper>>;
        fn num_records(self: &BinseqReaderWrapper) -> usize;
        unsafe fn get_record<'a>(
            self: &'a BinseqReaderWrapper,
            idx: usize,
        ) -> Result<Box<RecordWrapper<'a>>>;
        fn get_slen(self: &BinseqReaderWrapper) -> u32;
        fn get_xlen(self: &BinseqReaderWrapper) -> u32;

        // Record methods
        fn get_flag(self: &RecordWrapper<'_>) -> u64;
        fn is_paired(self: &RecordWrapper<'_>) -> bool;
        fn decode_s(self: &RecordWrapper<'_>, buffer: &mut Vec<u8>) -> Result<()>;
        fn decode_x(self: &RecordWrapper<'_>, buffer: &mut Vec<u8>) -> Result<()>;
    }
}

// Function to open a memory-mapped reader
pub fn open_mmap_reader(path: &str) -> binseq::Result<Box<BinseqReaderWrapper>> {
    let reader = MmapReader::new(Path::new(path))?;
    Ok(Box::new(BinseqReaderWrapper { reader }))
}

// Implementation for BinseqReaderWrapper
impl BinseqReaderWrapper {
    pub fn num_records(&self) -> usize {
        self.reader.num_records()
    }

    pub unsafe fn get_record<'a>(&'a self, idx: usize) -> binseq::Result<Box<RecordWrapper<'a>>> {
        let record = self.reader.get(idx)?;
        Ok(Box::new(RecordWrapper { record }))
    }

    pub fn get_slen(&self) -> u32 {
        self.reader.header().slen
    }

    pub fn get_xlen(&self) -> u32 {
        self.reader.header().xlen
    }
}

// Implementation for RecordWrapper
impl<'a> RecordWrapper<'a> {
    pub fn get_flag(&self) -> u64 {
        self.record.flag()
    }

    pub fn is_paired(&self) -> bool {
        self.record.paired()
    }

    pub fn decode_s(&self, buffer: &mut Vec<u8>) -> binseq::Result<()> {
        self.record.decode_s(buffer)
    }

    pub fn decode_x(&self, buffer: &mut Vec<u8>) -> binseq::Result<()> {
        self.record.decode_x(buffer)
    }
}

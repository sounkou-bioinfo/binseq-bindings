use binseq::{MmapReader, RefRecord};
use std::path::Path;

#[cxx::bridge]
mod ffi {
    // Shared types between Rust and C++
    #[derive(Debug, Copy, Clone)]
    struct BinseqHeaderInfo {
        slen: u32,
        xlen: u32,
        format_version: u8,
    }

    // Rust types and functions exposed to C++
    extern "Rust" {
        // Opaque types for C++
        type BinseqReader;
        type BinseqRecord;
        type BinseqBuffer;

        // Reader management
        fn new_binseq_reader(path: &str) -> Result<Box<BinseqReader>>;
        fn num_records(self: &BinseqReader) -> usize;
        fn header_info(self: &BinseqReader) -> BinseqHeaderInfo;
        fn get_record(self: &BinseqReader, idx: usize) -> Result<Box<BinseqRecord>>;

        // Record operations
        fn flag(self: &BinseqRecord) -> u64;
        fn is_paired(self: &BinseqRecord) -> bool;
        fn id(self: &BinseqRecord) -> usize;

        // Buffer management
        fn new_binseq_buffer() -> Box<BinseqBuffer>;

        // Decoding operations
        fn decode_primary(self: &BinseqRecord, buffer: &mut BinseqBuffer) -> Result<usize>;
        fn decode_extended(self: &BinseqRecord, buffer: &mut BinseqBuffer) -> Result<usize>;

        // Buffer access
        fn data(self: &BinseqBuffer) -> &[u8];
        fn data_mut(self: &mut BinseqBuffer) -> &mut [u8];
        fn clear(self: &mut BinseqBuffer);
    }
}

// Implementation of the wrapper types
pub struct BinseqReader {
    inner: MmapReader,
}

pub struct BinseqRecord {
    inner: RefRecord<'static>,
}

pub struct BinseqBuffer {
    buffer: Vec<u8>,
}

// Function implementations
fn new_binseq_reader(path: &str) -> binseq::Result<Box<BinseqReader>> {
    let reader = MmapReader::new(Path::new(path))?;
    Ok(Box::new(BinseqReader { inner: reader }))
}

impl BinseqReader {
    fn num_records(&self) -> usize {
        self.inner.num_records()
    }

    fn header_info(&self) -> ffi::BinseqHeaderInfo {
        let header = self.inner.header();
        ffi::BinseqHeaderInfo {
            slen: header.slen,
            xlen: header.xlen,
            format_version: header.format,
        }
    }

    fn get_record(&self, idx: usize) -> binseq::Result<Box<BinseqRecord>> {
        let record = self.inner.get(idx)?;
        let ref_record: RefRecord<'static> = unsafe { std::mem::transmute(record) };
        Ok(Box::new(BinseqRecord { inner: ref_record }))
    }
}

impl BinseqRecord {
    fn flag(&self) -> u64 {
        self.inner.flag()
    }

    fn is_paired(&self) -> bool {
        self.inner.paired()
    }

    fn id(&self) -> usize {
        self.inner.id()
    }

    fn decode_primary(&self, buffer: &mut BinseqBuffer) -> binseq::Result<usize> {
        buffer.clear();
        self.inner.decode_s(&mut buffer.buffer)?;
        Ok(buffer.buffer.len())
    }

    fn decode_extended(&self, buffer: &mut BinseqBuffer) -> binseq::Result<usize> {
        buffer.clear();
        self.inner.decode_x(&mut buffer.buffer)?;
        Ok(buffer.buffer.len())
    }
}

fn new_binseq_buffer() -> Box<BinseqBuffer> {
    Box::new(BinseqBuffer { buffer: Vec::new() })
}

impl BinseqBuffer {
    fn data(&self) -> &[u8] {
        &self.buffer
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }
}
